use super::r#type::Types;
use kstring::KString;

#[derive(Debug, Clone)]
pub struct Species {
    pub key: KString,
    pub name: KString,
    pub types: Types,
}
