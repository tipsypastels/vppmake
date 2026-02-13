pub mod pokemon;
pub mod profile;
pub mod room;
pub mod species;
pub mod r#type;

use self::{pokemon::Pokemon, profile::Profile, room::Room, species::Species};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use kstring::KString;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub profile: Profile,
    pub pokemon: Map<Pokemon>,
    pub rooms: Map<Room>,
}

pub type Map<T> = Arc<IndexMap<KString, T, ahash::RandomState>>;

pub trait MapFind<T: MapFindLabel> {
    fn find(&self, key: &str) -> Result<&T>;
}

impl<T: MapFindLabel> MapFind<T> for Map<T> {
    fn find(&self, key: &str) -> Result<&T> {
        self.get(key)
            .with_context(|| format!("Unknown {}: {}", T::LABEL, key))
    }
}

pub trait MapFindLabel {
    const LABEL: &'static str;
}

impl MapFindLabel for Pokemon {
    const LABEL: &'static str = "pokemon";
}

impl MapFindLabel for Species {
    const LABEL: &'static str = "species";
}
