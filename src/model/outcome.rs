use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    DryRun(String, String),
    Success(String, String),
    Failure(String, String),
}

#[derive(Debug)]
pub(crate) struct Outcomes {
    outcomes: BTreeMap<String, Vec<Outcome>>,
}

#[macro_export]
macro_rules! dry_run_or_failure {
    ($approve:expr, $param_1:expr, $param_2:expr) => {
        if $approve {
            Outcome::Failure($param_1.to_string(), $param_2.to_string())
        } else {
            Outcome::DryRun($param_1.to_string(), $param_2.to_string())
        }
    };
}

#[macro_export]
macro_rules! dry_run_or_success {
    ($approve:expr, $param_1:expr, $param_2:expr) => {
        if $approve {
            Outcome::Success($param_1.to_string(), $param_2.to_string())
        } else {
            Outcome::DryRun($param_1.to_string(), $param_2.to_string())
        }
    };
}

impl Outcomes {
    pub(crate) fn new() -> Outcomes {
        Outcomes {
            outcomes: BTreeMap::<String, Vec<Outcome>>::new(),
        }
    }

    pub(crate) fn add(&mut self, name: &String, outcomes: Vec<Outcome>) {
        self.outcomes
            .entry(name.to_owned())
            .or_default()
            .extend(outcomes);
    }

    pub(crate) fn _successful(&self, name: &String) -> bool {
        !self._failed(name)
    }

    pub(crate) fn _failed(&self, name: &String) -> bool {
        if let Some(outcomes) = self.outcomes.get(name) {
            outcomes.iter().any(|outcome| match outcome {
                Outcome::Failure(_, _) => true,
                Outcome::Success(_, _) | Outcome::DryRun(_, _) => false,
            })
        } else {
            false
        }
    }
}
