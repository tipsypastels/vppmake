pub mod pokemon;
pub mod profile;
pub mod room;
pub mod species;
pub mod r#type;

use self::{pokemon::Pokemon, profile::Profile, room::Room, r#type::Type};
use ahash::AHashMap;
use kstring::KString;
use serde::Deserialize;
use std::sync::Arc;

pub type Map<T> = Arc<AHashMap<KString, T>>;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub profile: Profile,
    pub types: Map<Type>,
    pub pokemon: Map<Pokemon>,
    pub rooms: Map<Room>,
}
