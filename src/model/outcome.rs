use std::collections::BTreeMap;

/// Each element in a recipe yields one outcome.
///
/// An outcome can be:
/// * a dry run - when we don't "--approve" the run
/// * a success - when the element from the recipe got successfully applied
/// * a failure - when some error occurred during the application of the element of the recipe
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    DryRun { block_id: String, message: String },
    Success { block_id: String, message: String },
    Failure { block_id: String, message: String },
}

/// A collection of [`Outcome`]s
#[derive(Debug)]
pub(crate) struct Outcomes {
    pub(crate) inner: BTreeMap<String, Vec<Outcome>>,
}

/// Macro to support "dry-run" outcomes in conjunction with [`Outcome::Failure`]
/// Returns a [`Outcome::DryRun`] for a "dry-run" (`$approve` = false) and a [`Outcome::Failure`] otherwise (`$approve` = true)
#[macro_export]
macro_rules! dry_run_or_failure {
    ($approve:expr, $block_id:expr, $message:expr) => {
        if $approve {
            Outcome::Failure {
                block_id: $block_id.to_string(),
                message: $message.to_string(),
            }
        } else {
            Outcome::DryRun {
                block_id: $block_id.to_string(),
                message: $message.to_string(),
            }
        }
    };
}

/// Macro to support "dry-run" outcomes in conjunction with [`Outcome::Success`]
/// Returns a [`Outcome::DryRun`] for a "dry-run" (`$approve` = false) and a [`Outcome::Success`] otherwise (`$approve` = true)
#[macro_export]
macro_rules! dry_run_or_success {
    ($approve:expr, $block_id:expr, $message:expr) => {
        if $approve {
            Outcome::Success {
                block_id: $block_id.to_string(),
                message: $message.to_string(),
            }
        } else {
            Outcome::DryRun {
                block_id: $block_id.to_string(),
                message: $message.to_string(),
            }
        }
    };
}

impl Outcomes {
    /// Create a new [`Outcome`] collection.
    /// Backed by a [`BTreeMap`] where the key is the name of the recipe
    /// and value is the list of outcomes.
    pub(crate) fn new() -> Outcomes {
        Outcomes {
            inner: BTreeMap::<String, Vec<Outcome>>::new(),
        }
    }

    /// Add a list of [`Outcome`]s for a given recipe name.
    /// If a recipe already has some [`Outcome`]s, we just append the new ones.
    pub(crate) fn add(&mut self, recipe_name: &String, outcomes: Vec<Outcome>) {
        self.inner
            .entry(recipe_name.to_owned())
            .or_default()
            .extend(outcomes);
    }
}
