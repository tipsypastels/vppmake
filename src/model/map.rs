use ahash::AHashMap;
use anyhow::{Context, Result};
use kstring::KString;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Map<T>(Arc<AHashMap<KString, T>>);

impl<T> Map<T> {
    pub fn get(&self, slug: &str) -> Result<&T> {
        self.0
            .get(slug)
            .with_context(|| format!("Unknown entry: '{slug}'"))
    }
}

impl<T> Clone for Map<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
