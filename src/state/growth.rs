use crate::model::{
    pokemon::{Pokemon, PokemonGrowth},
    species::Species,
};
use kstring::KString;

#[derive(Debug)]
pub enum GrowthState {
    Egg {
        hatch_at_posts: u16,
    },
    Growing {
        species: KString,
        grown_at_posts: u16,
    },
    Grown {
        species: KString,
    },
}

impl GrowthState {
    pub fn of(pokemon: &Pokemon, species: &Species, post_count: u16) -> Self {
        let grown = || Self::Grown {
            species: species.line.last().unwrap_or(&species.key).clone(),
        };

        let PokemonGrowth::Bounds(bounds) = pokemon.growth else {
            return grown();
        };

        assert!(post_count >= bounds.start);
        assert!(species.line.len() < 4);

        if post_count >= bounds.grown {
            return grown();
        }

        if post_count < bounds.hatch {
            return Self::Egg {
                hatch_at_posts: bounds.hatch,
            };
        }

        if species.line.len() < 3 {
            return Self::Growing {
                species: species.line[0].clone(),
                grown_at_posts: bounds.grown,
            };
        }

        let distance = (bounds.grown - bounds.hatch) as f32;
        let progress = (post_count - bounds.hatch) as f32;
        let ratio = progress / distance;

        Self::Growing {
            species: species.line[if ratio >= 0.5 { 1 } else { 0 }].clone(),
            grown_at_posts: bounds.grown,
        }
    }
}
