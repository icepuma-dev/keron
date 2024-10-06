use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long, value_name = "recipe_root")]
    pub(crate) recipe_root: PathBuf,
}
