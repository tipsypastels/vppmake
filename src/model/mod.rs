pub mod map;
pub mod pokemon;
pub mod profile;
pub mod room;
pub mod r#type;

use self::{map::Map, pokemon::Pokemon, profile::Profile, r#type::Type};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Source {
    profile: Profile,
    types: Map<Type>,
    pokemon: Map<Pokemon>,
}

impl Source {
    pub fn profile(&self) -> &Profile {
        &self.profile
    }
}
