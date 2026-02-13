use crate::model::profile::Profile;
use anyhow::{Context, Result};
use scraper::{Html, Selector};

pub async fn fetch(profile: &Profile) -> Result<u16> {
    let res = reqwest::get(&*profile.url).await?.error_for_status()?;
    let text = res.text().await?;
    let html = Html::parse_document(&text);
    let sel = Selector::parse(&profile.post_count_query).expect("invalid selector");
    let node = html.select(&sel).next().context("no match for selector")?;
    let text = node.text().next().context("selector has no text")?;
    let count = text.trim().replace(",", "").parse()?;

    eprintln!("Got post count '{count}'.");
    Ok(count)
}
