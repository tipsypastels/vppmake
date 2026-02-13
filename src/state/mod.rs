pub mod growth;

use crate::model::{Map, Source, species::Species};

#[derive(Debug, Clone)]
pub struct State {
    pub src: Source,
    pub species: Map<Species>,
    pub post_count: u16,
}
