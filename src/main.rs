use std::fs::canonicalize;

use clap::Parser;
use cli::{Apply, Options};
use recipe::{load_all_recipes, Engine};

mod cli;
mod model;
mod processor;
mod recipe;

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    match options.subcommand {
        cli::SubCommand::Apply(Apply { approve }) => {
            let recipe_root = canonicalize(options.recipe_root)?;

            let recipes = load_all_recipes(&recipe_root)?;

            let engine = Engine::new();

            let outcome = engine.run(approve, &recipes, &recipe_root)?;

            println!("{outcome:?}");
        }
    }

    Ok(())
}
