use clap::Parser;
use cli::Cli;
use dry_run::DryRun;
use recipe::RecipeResolver;

mod cli;
mod dry_run;
mod recipe;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let resolver = RecipeResolver::new();

    let recipes = resolver.resolve(&cli.recipe_root)?;

    let dry_run = DryRun::new();
    let reports = dry_run.run(&recipes)?;

    println!("{reports:#?}");

    Ok(())
}
