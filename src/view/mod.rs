mod assets;
mod bbcode;
mod color;
mod css;

use self::{bbcode::BBCode, color::color, css::Css};
use crate::{model::MapFind, state::State};
use anyhow::Result;
use assets::room_asset;

const SPARKLE: &str = "url('/images/sparkle.gif')";
const TABS_WIDTH: usize = 1000;

pub fn render(state: &State) -> Result<String> {
    let mut bb = BBCode::new();

    bb.tag_with("tabs", TABS_WIDTH, |bb| render_room(bb, state))?;

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
