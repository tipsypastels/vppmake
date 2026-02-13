use super::bbcode::BBCode;
use crate::{
    model::{MapFindPokemon, MapFindSpecies, room::Room},
    state::State,
};
use anyhow::Result;

pub fn render(bb: &mut BBCode, state: &State, room: &Room) -> Result<()> {
    bb.tag_with("room", &room.key, |bb| {
        bb.tag("room-name", |bb| {
            bb.text(&room.name);
        });

        for placement in room.pokemon.values() {
            let pokemon = state.src.pokemon.find(&placement.key)?;

            bb.tag_with("pokemon", &pokemon.name, |bb| {
                let growth = state.growth_state(pokemon)?;
                if let Some(species_key) = growth.species_key() {
                    let species = state.species.find(species_key)?;
                    bb.text(&species.name);
                } else {
                    bb.text("egg");
                }
                Ok(())
            })?;
        }
        Ok(())
    })?;

    Ok(())
}
