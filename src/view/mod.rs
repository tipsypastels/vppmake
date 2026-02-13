mod assets;
mod bbcode;
mod css;

use self::{bbcode::BBCode, css::Css};
use crate::{model::MapFind, state::State};
use anyhow::Result;
use assets::room_asset;

const SPARKLE: &str = "url('/images/sparkle.gif')";

pub fn render(state: &State) -> Result<String> {
    let mut bb = BBCode::new();

    bb.tag("tabs", |bb| render_room(bb, state))?;

    Ok(bb.to_string())
}

pub fn render_room(bb: &mut BBCode, state: &State) -> Result<()> {
    // bb.tag_with("room", &room.key, |bb| {
    //     bb.tag("room-name", |bb| {
    //         bb.text(&room.name);
    //     });

    //     for placement in room.pokemon.values() {
    //         let pokemon = state.src.pokemon.find(&placement.key)?;

    //         bb.tag_with("pokemon", &pokemon.name, |bb| {
    //             let growth = state.growth_state(pokemon)?;
    //             if let Some(species_key) = growth.species_key() {
    //                 let species = state.species.find(species_key)?;
    //                 bb.text(&species.name);
    //             } else {
    //                 bb.text("egg");
    //             }
    //             Ok(())
    //         })?;
    //     }
    //     Ok(())
    // })?;

    // Ok(())

    for room in state.src.rooms.values() {
        bb.tag_with("slide", &room.name, |bb| {
            bb.tag("img", |bb| {
                bb.text(room_asset(&room.key));
            });

            for placement in room.pokemon.values() {
                let pokemon = state.src.pokemon.find(&placement.key)?;
                let growth = state.growth_state(pokemon)?;

                let sprite_css = Css::new()
                    .set("position", "absolute")
                    .setopt(placement.flipped, "transform", "scaleX(-1)")
                    .setopt(pokemon.shiny, "background-image", SPARKLE)
                    .extend(placement.position.unpack());

                bb.tag_with("div", sprite_css, |bb| {
                    bb.text(&pokemon.name);
                });
            }
            Ok(())
        })?;
    }
    Ok(())
}
