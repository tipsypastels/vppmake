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

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum PokemonGrowth {
    Stage(PokemonGrowthStage),
    Bounds(PokemonGrowthBounds),
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PokemonGrowthStage {
    Egg,
    Growing { level: u8 },
    Grown,
}

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct PokemonGrowthBounds {
    start: u16,
    hatch: u16,
    grown: u16,
}
