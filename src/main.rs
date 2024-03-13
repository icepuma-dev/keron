use std::fs::canonicalize;

use clap::Parser;
use cli::{Apply, Options};
use colored::Colorize;
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

            let outcomes = engine.run(approve, &recipes, &recipe_root)?;

            for (key, value) in outcomes.inner {
                println!("{}:", key.underline().bold());
                println!();

                for outcome in value {
                    match outcome {
                        model::Outcome::DryRun { block_id, message } => {
                            println!("⏸️ - {block_id}: {message}");
                        }
                        model::Outcome::Success { block_id, message } => {
                            println!("✅ - {block_id}: {message}");
                        }
                        model::Outcome::Failure { block_id, message } => {
                            println!("❌ - {block_id}: {message}");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
