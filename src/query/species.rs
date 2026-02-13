use crate::model::{
    Map, Source,
    pokemon::PokemonSpecies,
    species::Species,
    r#type::{Type, Types},
};
use ahash::AHashMap;
use anyhow::{Context, Result, bail};
use kstring::KString;
use rustemon::{
    Follow, client::RustemonClient as Client, model::evolution as api_evo, model::pokemon as api,
    pokemon::pokemon,
};
use smallvec::SmallVec;
use std::sync::Arc;
use tokio::sync::mpsc;

// NOTE: In PokeAPI, "species" are the base pokemon and "pokemon" are variations such as "-gmax", etc.
// We don't use that distinction, we count every form variation as a distinct "species".

type Line<T> = SmallVec<[T; 3]>;

pub async fn fetch(src: &Source) -> Result<Map<Species>> {
    let (fetcher, mut rx) = Fetcher::new(src.types.clone());
    let mut out = AHashMap::new();

    for pokemon in src.pokemon.values() {
        fetcher.fetch(pokemon.species.clone());
    }

    drop(fetcher); // kill the sender

    while let Some(line) = rx.recv().await {
        for species in line? {
            out.insert(species.key.clone(), species);
        }
    }

    dbg!(&out);

    Ok(Arc::new(out))
}

#[derive(Clone)]
struct Fetcher {
    client: Arc<Client>,
    tx: mpsc::Sender<Result<Line<Species>>>,
    types: Map<Type>,
}

impl Fetcher {
    fn new(types: Map<Type>) -> (Self, mpsc::Receiver<Result<Line<Species>>>) {
        let client = Arc::new(Client::default());
        let (tx, rx) = mpsc::channel(4);
        (Self { client, tx, types }, rx)
    }

    fn fetch(&self, species_info: PokemonSpecies) {
        let this = self.clone();

        tokio::spawn(async move {
            match this.fetch_inner(species_info).await {
                Ok(line) => {
                    let _ = this.tx.send(Ok(line)).await;
                }
                Err(error) => {
                    let _ = this.tx.send(Err(error)).await;
                }
            };
        });
    }

    async fn fetch_inner(&self, species_info: PokemonSpecies) -> Result<Line<Species>> {
        let (from_key, to_key) = match &species_info {
            PokemonSpecies::Exact { species } => (species, None),
            PokemonSpecies::Line { from, to } => (from, Some(to)),
        };

        let (from_mon, from_mon_api_species) = self.fetch_one(from_key.clone()).await?;

        let Some(to_key) = to_key else {
            return Ok(std::iter::once(from_mon).collect());
        };

        let Some(evo_chain_res) = &from_mon_api_species.evolution_chain else {
            bail!("Species {from_key} has no evolution chain.");
        };

        let evo_chain = evo_chain_res.follow(&self.client).await?;
        let evo_keys = self.follow_evo_chain(&evo_chain.chain.evolves_to, to_key.as_deref())?;
        let mut evo_mons = Line::new();

        for evo_key in evo_keys {
            evo_mons.push(self.fetch_one(evo_key).await?.0);
        }

        Ok(std::iter::once(from_mon)
            .chain(evo_mons.into_iter())
            .collect())
    }

    async fn fetch_one(&self, key: KString) -> Result<(Species, api::PokemonSpecies)> {
        let pokemon = pokemon::get_by_name(&key, &self.client).await?;
        let species = pokemon.species.follow(&self.client).await?;

        let name = self.fetch_name(&pokemon, &species).await?;
        let types = self.fetch_types(&pokemon.types)?;

        println!("Got species '{key}'.");
        Ok((Species { key, name, types }, species))
    }

    async fn fetch_name(
        &self,
        pokemon: &api::Pokemon,
        species: &api::PokemonSpecies,
    ) -> Result<KString> {
        macro_rules! en_name {
            ($res:expr) => {
                $res.names
                    .iter()
                    .find(|n| n.language.name == "en")
                    .map(|n| KString::from_ref(&n.name))
            };
        }
        if let Some(form) = pokemon.forms.first()
            && let Some(name) = en_name!(form.follow(&self.client).await?)
        {
            return Ok(name);
        };
        en_name!(species).with_context(|| format!("Unknown pokemon name: '{}'", pokemon.name))
    }

    fn fetch_types(&self, types: &[api::PokemonType]) -> Result<Types> {
        let mut iter = types.iter().map(|t| {
            self.types
                .get(&*t.type_.name)
                .cloned()
                .with_context(|| format!("Invalid type: '{}'", t.type_.name))
        });
        let one = iter.next().context("Expected at least one type")??;
        let types = if let Some(two) = iter.next() {
            Types::Two(one, two?)
        } else {
            Types::One(one)
        };
        if iter.next().is_some() {
            bail!("Expected no more than two types");
        }
        Ok(types)
    }

    fn follow_evo_chain(
        &self,
        links: &[api_evo::ChainLink],
        target: Option<&str>,
    ) -> Result<Line<KString>> {
        fn no_branches(mut links: &[api_evo::ChainLink]) -> bool {
            while !links.is_empty() {
                if links.len() > 1 {
                    return false;
                }
                links = &links[0].evolves_to
            }
            true
        }

        fn build_no_branches(mut links: &[api_evo::ChainLink]) -> Line<KString> {
            let mut v = Line::new();
            while !links.is_empty() {
                v.push(KString::from_ref(&links[0].species.name));
                links = &links[0].evolves_to;
            }
            v
        }

        if no_branches(links) {
            return Ok(build_no_branches(links));
        }
        let Some(target) = target else {
            bail!("Target required for branching evolution chain.");
        };

        fn visit_branch_link(
            target: &str,
            link: &api_evo::ChainLink,
            indices: Line<usize>,
        ) -> Option<Line<usize>> {
            if link.species.name == target {
                return Some(indices);
            }
            visit_branch_links(target, &link.evolves_to, indices)
        }

        fn visit_branch_links(
            target: &str,
            links: &[api_evo::ChainLink],
            parent_indices: Line<usize>,
        ) -> Option<Line<usize>> {
            links.iter().enumerate().find_map(|(i, link)| {
                let mut indices = parent_indices.clone();
                indices.push(i);
                visit_branch_link(target, link, indices)
            })
        }

        let Some(indices) = visit_branch_links(target, links, Line::new()) else {
            bail!("Could not find path to {target}.");
        };

        let mut out = Line::new();
        let mut link = &links[indices[0]];
        out.push(KString::from_ref(&link.species.name));

        for &index in indices.iter().skip(1) {
            link = &link.evolves_to[index];
            out.push(KString::from_ref(&link.species.name));
        }

        Ok(out)
    }
}
