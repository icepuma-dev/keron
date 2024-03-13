use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, about, version)]
pub(crate) struct Options {
    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,

    #[arg(long)]
    pub(crate) recipe_root: PathBuf,
}

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    #[command(about = "Apply")]
    Apply(Apply),
}

#[derive(Parser, Debug)]
pub(crate) struct Apply {
    #[arg(long)]
    pub(crate) approve: bool,
}
