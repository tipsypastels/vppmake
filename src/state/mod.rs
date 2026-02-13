pub mod growth;

use self::growth::GrowthState;
use crate::model::{Map, MapFind, Source, pokemon::Pokemon, species::Species};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct State {
    pub src: Source,
    pub species: Map<Species>,
    pub post_count: u16,
}

impl State {
    pub fn growth_state(&self, pokemon: &Pokemon) -> Result<GrowthState> {
        let species = self.species.find(pokemon.species.from_key())?;
        Ok(GrowthState::of(pokemon, species, self.post_count))
    }
}
