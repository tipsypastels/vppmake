mod assets;
mod bbcode;
mod css;
mod unit;

use self::{bbcode::BBCode, css::Css, unit::absolute};
use crate::{
    model::{
        MapFind,
        room::{Room, RoomMember},
    },
    state::State,
};
use anyhow::Result;
use assets::{egg_asset, pokemon_asset, room_asset, type_icon_asset};
use unit::Px;

const SPARKLE: &str = "url('/images/sparkle.gif')";
const TABS_WIDTH: usize = 500;
const TABS_HEADER_PADDING: &str = "0.25rem 0.5rem";
const TYPE_ICON_SIZE: Px = Px(20);

pub fn render(state: &State) -> Result<String> {
    let mut bb = BBCode::new();

    bb.tag_with_multi(
        "tabs",
        (("width", TABS_WIDTH), ("block_align", "bcenter")),
        |bb| render_rooms(bb, state),
    )?;

    Ok(bb.to_string())
}

pub fn render_rooms(bb: &mut BBCode, state: &State) -> Result<()> {
    for room in state.src.rooms.values() {
        render_room(bb, state, room)?;
    }
    Ok(())
}

fn render_room(bb: &mut BBCode, state: &State, room: &Room) -> Result<()> {
    bb.tag("slide_header", |bb| {
        bb.tag_with("div", Css::new().set("margin", TABS_HEADER_PADDING), |bb| {
            bb.text(&room.name);
        });
    })
    .tag("slide", |bb| {
        bb.tag_with("div", Css::new().set("position", "relative"), |bb| {
            bb.tag_with("cimg", Css::new().set("user-select", "none"), |bb| {
                bb.text(room_asset(&room.key));
            });
            for member in room.pokemon.values() {
                render_room_member_sprite(bb, state, member)?;
            }
            Ok(())
        })?
        .tag_with_multi("tabs", (("width", "100%"),), |bb| {
            for member in room.pokemon.values() {
                render_room_member_info(bb, state, member)?;
            }
            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}

fn render_room_member_sprite(bb: &mut BBCode, state: &State, member: &RoomMember) -> Result<()> {
    let pokemon = state.src.pokemon.find(&member.key)?;
    let growth = state.growth_state(pokemon)?;
    let species = growth.species_key();

    let sprite_css = Css::new()
        .set("cursor", "default")
        .set("user-select", "none")
        .set("position", "absolute")
        .setopt(member.flipped, "transform", "scaleX(-1)")
        .setopt(pokemon.shiny, "background-image", SPARKLE)
        .extend(absolute(member.position.unpack()));

    bb.tag_with("title", &pokemon.name, |bb| {
        bb.tag_with("cimg", sprite_css, |bb| {
            if let Some(species) = species {
                bb.text(pokemon_asset(&pokemon.key, species));
            } else {
                bb.text(egg_asset());
            }
        });
    });

    Ok(())
}

fn render_room_member_info(bb: &mut BBCode, state: &State, member: &RoomMember) -> Result<()> {
    let pokemon = state.src.pokemon.find(&member.key)?;
    let growth = state.growth_state(pokemon)?;
    let species = growth
        .species_key()
        .map(|key| state.species.find(key))
        .transpose()?;

    bb.tag("slide_header", |bb| {
        bb.tag_with("div", Css::new().set("margin", TABS_HEADER_PADDING), |bb| {
            bb.text(&pokemon.name);
        });
    })
    .tag("slide", |bb| {
        bb.tag_with(
            "div",
            Css::new()
                .set("display", "flex")
                .set("margin-bottom", "0.5rem"),
            |bb| {
                bb.tag_with(
                    "div",
                    Css::new()
                        .set("flex-grow", "1")
                        .set("font-weight", "bold")
                        .set("font-size", "1.25rem"),
                    |bb| {
                        bb.text("\"")
                            .text(&pokemon.name)
                            .text("\" the ")
                            .text(species.map(|s| s.name.as_str()).unwrap_or("Egg"));
                    },
                )
                .tag_with("div", Css::new(), |bb| {
                    if let Some(species) = species {
                        for r#type in &species.types {
                            bb.tag_with("title", &r#type.name, |bb| {
                                bb.tag_with(
                                    "cimg",
                                    Css::new()
                                        .set("cursor", "default")
                                        .set("user-select", "none")
                                        .set("width", TYPE_ICON_SIZE)
                                        .set("height", TYPE_ICON_SIZE)
                                        .set("margin-left", "0.5rem"),
                                    |bb| {
                                        bb.text(type_icon_asset(&r#type.key));
                                    },
                                );
                            });
                        }
                    }
                });
            },
        )
        .text(format!("{pokemon:?}"));
    });

    Ok(())
}
