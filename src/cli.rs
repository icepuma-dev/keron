use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Command line options.
#[derive(Parser)]
#[command(author, about, version)]
pub(crate) struct Options {
    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,

    #[arg(long)]
    pub(crate) recipe_root: PathBuf,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    #[command(about = "Apply")]
    Apply(Apply),
}

/// The "apply" subcommand.
///
/// If `approve` is `true` we try to apply all changes.
#[derive(Parser, Debug)]
pub(crate) struct Apply {
    #[arg(long)]
    pub(crate) approve: bool,
}
