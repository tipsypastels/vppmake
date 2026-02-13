pub mod pokemon;
pub mod profile;
pub mod room;
pub mod species;
pub mod r#type;

use self::{pokemon::Pokemon, profile::Profile, room::Room, species::Species, r#type::Type};
use ahash::AHashMap;
use anyhow::{Context, Result};
use kstring::KString;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub profile: Profile,
    pub types: Map<Type>,
    pub pokemon: Map<Pokemon>,
    pub rooms: Map<Room>,
}

pub type Map<T> = Arc<AHashMap<KString, T>>;

macro_rules! map_access_traits {
    ($(($trait:ident, $ty:ty, $label:literal)),*$(,)?) => {
        $(
            pub trait $trait {
                fn find(&self, key: &str) -> Result<&$ty>;
            }

            impl $trait for Map<$ty> {
                fn find(&self, key: &str) -> Result<&$ty> {
                    self.get(key).with_context(|| format!(concat!("Unknown ", $label, ": {}"), key))
                }
            }
        )*
    };
}

map_access_traits! {
    (MapFindPokemon, Pokemon, "pokemon"),
    (MapFindSpecies, Species, "species"),
    (MapFindType, Type, "type"),
}
