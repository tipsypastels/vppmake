mod assets;
mod bbcode;
mod css;
mod unit;

use self::{
    bbcode::BBCode,
    css::Css,
    unit::{absolute, color},
};
use crate::{model::MapFind, state::State};
use anyhow::Result;
use assets::{egg_asset, pokemon_asset, room_asset};

const SPARKLE: &str = "url('/images/sparkle.gif')";
const TABS_WIDTH: usize = 500;

pub fn render(state: &State) -> Result<String> {
    let mut bb = BBCode::new();

    bb.tag_with_multi(
        "tabs",
        (("width", TABS_WIDTH), ("block_align", "bcenter")),
        |bb| render_room(bb, state),
    )?;

    Ok(bb.to_string())
}

pub fn render_room(bb: &mut BBCode, state: &State) -> Result<()> {
    for room in state.src.rooms.values() {
        bb.tag("slide_header", |bb| {
            bb.tag_with(
                "div",
                Css::new()
                    .set("margin", "0.25rem 0.5rem")
                    .set("font-weight", "bold")
                    .set("color", color(room.color)),
                |bb| {
                    bb.text(&room.name);
                },
            );
        });

        bb.tag("slide", |bb| {
            bb.tag_with("div", Css::new().set("position", "relative"), |bb| {
                bb.tag_with("cimg", "user-select:none", |bb| {
                    bb.text(room_asset(&room.key));
                });

                for placement in room.pokemon.values() {
                    let pokemon = state.src.pokemon.find(&placement.key)?;
                    let growth = state.growth_state(pokemon)?;
                    let species = growth.species_key();

                    let sprite_css = Css::new()
                        .set("position", "absolute")
                        .setopt(placement.flipped, "transform", "scaleX(-1)")
                        .setopt(pokemon.shiny, "background-image", SPARKLE)
                        .extend(absolute(placement.position.unpack()));

                    bb.tag_with("cimg", sprite_css, |bb| {
                        if let Some(species) = species {
                            bb.text(pokemon_asset(&pokemon.key, species));
                        } else {
                            bb.text(egg_asset());
                        }
                    });
                }
                Ok(())
            })?;
            Ok(())
        })?;
    }
    Ok(())
}
