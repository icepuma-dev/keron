use std::sync::{Arc, RwLock};

use rhai::Engine;

use crate::recipe::Recipe;

#[derive(Debug, Clone)]
pub(crate) struct DryRunReport {
    pub(crate) _recipe: Recipe,
    pub(crate) _recipe_output: Vec<String>,
}

pub(crate) struct DryRun;

impl DryRun {
    pub(crate) fn new() -> Self {
        DryRun {}
    }

    pub(crate) fn run(&self, recipes: &Vec<Recipe>) -> anyhow::Result<Vec<DryRunReport>> {
        let mut engine = Engine::new();

        let mut reports = vec![];

        for recipe in recipes {
            let recipe_output = Arc::new(RwLock::new(Vec::<String>::new()));

            let recipe_debug_output = recipe_output.clone();
            engine.on_debug(move |text, _, _| {
                recipe_debug_output.write().unwrap().push(text.to_string());
            });

            let recipe_print_output = recipe_output.clone();
            engine.on_print(move |text| {
                recipe_print_output.write().unwrap().push(text.to_string());
            });

            match engine.run(&recipe.content) {
                Ok(_) => {
                    reports.push(DryRunReport {
                        _recipe: recipe.clone(),
                        _recipe_output: recipe_output.read().unwrap().to_vec(),
                    });
                }
                Err(_) => todo!(),
            }
        }

        Ok(reports)
    }
}
