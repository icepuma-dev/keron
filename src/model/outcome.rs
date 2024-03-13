use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    DryRun { block_id: String, message: String },
    Success { block_id: String, message: String },
    Failure { block_id: String, message: String },
}

#[derive(Debug)]
pub(crate) struct Outcomes {
    pub(crate) inner: BTreeMap<String, Vec<Outcome>>,
}

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
    pub(crate) fn new() -> Outcomes {
        Outcomes {
            inner: BTreeMap::<String, Vec<Outcome>>::new(),
        }
    }

    pub(crate) fn add(&mut self, name: &String, outcomes: Vec<Outcome>) {
        self.inner
            .entry(name.to_owned())
            .or_default()
            .extend(outcomes);
    }

    pub(crate) fn _successful(&self, name: &String) -> bool {
        !self._failed(name)
    }

    pub(crate) fn _failed(&self, name: &String) -> bool {
        if let Some(outcomes) = self.inner.get(name) {
            outcomes.iter().any(|outcome| match outcome {
                Outcome::Failure { .. } => true,
                Outcome::Success { .. } | Outcome::DryRun { .. } => false,
            })
        } else {
            false
        }
    }
}
