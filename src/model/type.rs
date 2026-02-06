use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Type {
    key: KString,
    name: KString,
    icon: KString,
    color: [u8; 3],
}
