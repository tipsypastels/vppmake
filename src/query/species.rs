use crate::model::{
    Map, Source,
    pokemon::PokemonSpecies,
    species::{Species, SpeciesLine},
    r#type::{Type, Types},
};
use anyhow::{Context, Result, bail};
use futures::StreamExt;
use indexmap::IndexMap;
use kstring::KString;
use rustemon::{
    Follow, client::RustemonClient as Client, model::evolution as api_evo, model::pokemon as api,
    pokemon::pokemon,
};
use std::{pin::pin, sync::Arc};
use tokio::sync::mpsc;

// NOTE: In PokeAPI, "species" are the base pokemon and "pokemon" are variations such as "-gmax", etc.
// We don't use that distinction, we count every form variation as a distinct "species".

pub async fn fetch(src: &Source) -> Result<Map<Species>> {
    let (fetcher, mut rx) = Fetcher::new();
    let mut out = IndexMap::with_hasher(ahash::RandomState::new());

    for pokemon in src.pokemon.values() {
        fetcher.fetch(pokemon.species.clone());
    }

    drop(fetcher); // kill the sender

    while let Some(line) = rx.recv().await {
        for species in line? {
            out.insert(species.key.clone(), species);
        }
    }

    Ok(Arc::new(out))
}

#[derive(Clone)]
struct Fetcher {
    client: Arc<Client>,
    tx: mpsc::Sender<Result<SpeciesLine<Species>>>,
}

impl Fetcher {
    fn new() -> (Self, mpsc::Receiver<Result<SpeciesLine<Species>>>) {
        let client = Arc::new(Client::default());
        let (tx, rx) = mpsc::channel(4);
        (Self { client, tx }, rx)
    }

    fn fetch(&self, species_info: PokemonSpecies) {
        let this = self.clone();

        tokio::spawn(async move {
            let res = this.fetch_inner(species_info).await;
            let _ = this.tx.send(res).await;
        });
    }

    async fn fetch_inner(&self, species_info: PokemonSpecies) -> Result<SpeciesLine<Species>> {
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
        let mut evo_mons = SpeciesLine::new();

        for evo_key in &evo_keys {
            evo_mons.push(self.fetch_one(evo_key.clone()).await?.0);
        }

        let line_keys = std::iter::once(from_key.clone())
            .chain(evo_keys.into_iter())
            .collect::<SpeciesLine<KString>>();

        let mut line = std::iter::once(from_mon)
            .chain(evo_mons.into_iter())
            .collect::<SpeciesLine<Species>>();

        for species in &mut line {
            species.line = line_keys.clone();
        }

        Ok(line)
    }

    async fn fetch_one(&self, key: KString) -> Result<(Species, api::PokemonSpecies)> {
        let pokemon = pokemon::get_by_name(&key, &self.client).await?;
        let species = pokemon.species.follow(&self.client).await?;
        let (name, types) = tokio::try_join!(
            self.fetch_name(&pokemon, &species),
            self.fetch_types(&pokemon.types)
        )?;

        eprintln!("Got species '{key}'.");
        Ok((
            Species {
                key,
                name,
                types,
                // Will get overridden by caller.
                line: SpeciesLine::new(),
            },
            species,
        ))
    }

    async fn fetch_name(
        &self,
        pokemon: &api::Pokemon,
        species: &api::PokemonSpecies,
    ) -> Result<KString> {
        if let Some(form) = pokemon.forms.first()
            && let Some(name) = en_name_of!(form.follow(&self.client).await?)
        {
            return Ok(name);
        };
        en_name_of!(species).with_context(|| format!("No name for pokemon: '{}'", pokemon.name))
    }

    async fn fetch_types(&self, api_types: &[api::PokemonType]) -> Result<Types> {
        let stream = futures::stream::iter(api_types).then(|api_type| async move {
            let key = KString::from_ref(&api_type.type_.name);
            let res = api_type.type_.follow(&self.client).await?;
            let name = en_name_of!(res).with_context(|| format!("No name for type: '{key}'"))?;
            anyhow::Ok(Type { key, name })
        });
        let mut stream = pin!(stream);

        let one = stream
            .next()
            .await
            .context("Expected at least one type.")??;
        let types = if let Some(two) = stream.next().await {
            Types::Two(one, two?)
        } else {
            Types::One(one)
        };
        if stream.next().await.is_some() {
            bail!("Expected no more than two types.")
        }
        Ok(types)
    }

    fn follow_evo_chain(
        &self,
        links: &[api_evo::ChainLink],
        target: Option<&str>,
    ) -> Result<SpeciesLine<KString>> {
        fn no_branches(mut links: &[api_evo::ChainLink]) -> bool {
            while !links.is_empty() {
                if links.len() > 1 {
                    return false;
                }
                links = &links[0].evolves_to
            }
            true
        }

        fn build_no_branches(mut links: &[api_evo::ChainLink]) -> SpeciesLine<KString> {
            let mut v = SpeciesLine::new();
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
            indices: SpeciesLine<usize>,
        ) -> Option<SpeciesLine<usize>> {
            if link.species.name == target {
                return Some(indices);
            }
            visit_branch_links(target, &link.evolves_to, indices)
        }

        fn visit_branch_links(
            target: &str,
            links: &[api_evo::ChainLink],
            parent_indices: SpeciesLine<usize>,
        ) -> Option<SpeciesLine<usize>> {
            links.iter().enumerate().find_map(|(i, link)| {
                let mut indices = parent_indices.clone();
                indices.push(i);
                visit_branch_link(target, link, indices)
            })
        }

        let Some(indices) = visit_branch_links(target, links, SpeciesLine::new()) else {
            bail!("Could not find path to {target}.");
        };

        let mut out = SpeciesLine::new();
        let mut link = &links[indices[0]];
        out.push(KString::from_ref(&link.species.name));

        for &index in indices.iter().skip(1) {
            link = &link.evolves_to[index];
            out.push(KString::from_ref(&link.species.name));
        }

        Ok(out)
    }
}

macro_rules! en_name_of {
    ($res:expr) => {
        $res.names
            .iter()
            .find(|n| n.language.name == "en")
            .map(|n| KString::from_ref(&n.name))
    };
}
use en_name_of;
