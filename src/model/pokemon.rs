use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Pokemon {
    pub key: KString,
    pub name: KString,
    pub species: PokemonSpecies,
    pub growth: PokemonGrowth,
    #[serde(default)]
    pub shiny: bool,
    pub points: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PokemonSpecies {
    Exact {
        #[serde(rename = "exact")]
        species: KString,
    },
    Line {
        #[serde(rename = "line")]
        from: KString,
        #[serde(default)]
        to: Option<KString>,
    },
}

impl PokemonSpecies {
    pub fn from_key(&self) -> &KString {
        match self {
            Self::Exact { species } => species,
            Self::Line { from, .. } => from,
        }
    }

    pub fn to_key(&self) -> Option<&KString> {
        match self {
            Self::Exact { .. } => None,
            Self::Line { to, .. } => to.as_ref(),
        }
    }
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum PokemonGrowth {
    Grown(PokemonGrowthGrown),
    Bounds(PokemonGrowthBounds),
}

// Only here so serde can parse it as a string literal "grown".
#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PokemonGrowthGrown {
    Grown,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct PokemonGrowthBounds {
    pub start: u16,
    pub hatch: u16,
    pub grown: u16,
}
