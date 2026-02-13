use std::iter;

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

impl Types {
    pub fn iter(&self) -> TypesIter {
        match self {
            Self::One(r#type) => TypesIter::One(iter::once(r#type)),
            Self::Two(type1, type2) => TypesIter::Two(iter::once(type1).chain(iter::once(type2))),
        }
    }
}

impl<'a> IntoIterator for &'a Types {
    type Item = &'a Type;
    type IntoIter = TypesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub enum TypesIter<'a> {
    One(iter::Once<&'a Type>),
    Two(iter::Chain<iter::Once<&'a Type>, iter::Once<&'a Type>>),
}

impl<'a> Iterator for TypesIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::One(iter) => iter.next(),
            Self::Two(iter) => iter.next(),
        }
    }
}
