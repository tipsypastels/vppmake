use super::r#type::Types;
use kstring::KString;
use smallvec::SmallVec;

pub type SpeciesLine<T> = SmallVec<[T; 3]>;

#[derive(Debug, Clone)]
pub struct Species {
    pub key: KString,
    pub name: KString,
    pub types: Types,
    pub line: SpeciesLine<KString>,
}
