use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Type {
    pub key: KString,
    pub name: KString,
    pub icon: KString,
    pub color: [u8; 3],
}

#[derive(Debug, Clone)]
pub enum Types {
    One(Type),
    Two(Type, Type),
}
