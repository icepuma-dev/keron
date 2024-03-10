use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    model::{Outcomes, Recipe},
    processor::link_processor::LinkProcessor,
};

pub(crate) struct Engine {}

impl Engine {
    pub(crate) fn new() -> Engine {
        Engine {}
    }

    pub(crate) fn run(
        &self,
        _approve: bool,
        recipes: &BTreeMap<String, Recipe>,
        recipe_root: &PathBuf,
    ) -> anyhow::Result<Outcomes> {
        let mut outcomes = Outcomes::new();
        let link_processor = LinkProcessor::new();

        for (name, recipe) in recipes {
            if let Some(link) = &recipe.link {
                let link_outcomes = link_processor.process(recipe_root, link);
                outcomes.add(name, link_outcomes);
            }
        }

        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {}
