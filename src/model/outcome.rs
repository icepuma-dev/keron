use std::collections::BTreeMap;

#[derive(Debug)]
pub(crate) enum Outcome {
    Success(String, String),
    Failure(String, String),
}

#[derive(Debug)]
pub(crate) struct Outcomes {
    outcomes: BTreeMap<String, Vec<Outcome>>,
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
                Outcome::Success(_, _) => false,
            })
        } else {
            false
        }
    }
}
