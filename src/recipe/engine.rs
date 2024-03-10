use std::{collections::BTreeMap, path::Path};

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
        approve: bool,
        recipes: &BTreeMap<String, Recipe>,
        recipe_root: &Path,
    ) -> anyhow::Result<Outcomes> {
        let mut outcomes = Outcomes::new();
        let link_processor = LinkProcessor::new();

        for (name, recipe) in recipes {
            if let Some(link) = &recipe.link {
                let link_outcomes = link_processor.process(approve, recipe_root, link);
                outcomes.add(name, link_outcomes);
            }
        }

        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {}
