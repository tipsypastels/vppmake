use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub url: KString,
    pub post_count_query: KString,
}
