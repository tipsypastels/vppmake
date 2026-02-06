use kstring::KString;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    url: KString,
    post_count_query: KString,
}

impl Profile {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn post_count_query(&self) -> &str {
        &self.post_count_query
    }
}
