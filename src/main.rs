mod model;
mod query;
mod state;
mod view;

use self::{model::Source, state::State};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Make { src: Box<Path> },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let Command::Make { src: src_path } = cli.command;

    let src_text = fs_err::tokio::read_to_string(&src_path).await?;
    let src_text = add_keys_to_table_values(&src_text)?;
    let src: Source = toml::from_str(&src_text)?;

    let (species, post_count) = tokio::try_join!(
        query::species::fetch(&src),
        query::post_count::fetch(&src.profile),
    )?;

    let state = State {
        src,
        species,
        post_count,
    };

    dbg!(state);

    Ok(())
}

fn add_keys_to_table_values(src_text: &str) -> Result<String> {
    use toml_edit::{DocumentMut, Item, Table, visit_mut::*};
    let mut document = src_text.parse::<DocumentMut>()?;

    struct AddKeys;
    impl VisitMut for AddKeys {
        fn visit_table_mut(&mut self, table: &mut Table) {
            if table.is_implicit() {
                for (key, value) in table.iter_mut() {
                    let Item::Table(table) = value else {
                        continue;
                    };
                    table["key"] = key.get().into();
                }
            }

            visit_table_mut(self, table);
        }
    }

    AddKeys.visit_document_mut(&mut document);
    Ok(document.to_string())
}
