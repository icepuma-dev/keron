use std::path::PathBuf;

use indexmap::IndexMap;

use crate::model::{Link, Outcome};

pub(crate) struct LinkProcessor;

impl LinkProcessor {
    pub(crate) fn new() -> LinkProcessor {
        LinkProcessor {}
    }

    pub(crate) fn process(
        &self,
        recipe_root: &PathBuf,
        link: &IndexMap<PathBuf, Link>,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        for (source, link) in link {
            outcomes.extend(self.symlink(recipe_root, source, &link.to));
        }

        outcomes
    }

    fn resolve_path(&self, path: &PathBuf) -> PathBuf {
        let path = path.to_owned();

        if path.starts_with("~/") {
            // FIXME: what to do when this is empty
            let home_dir = dirs::home_dir().unwrap_or_default();

            home_dir.join(path.display().to_string().replace("~/", ""))
        } else {
            path.to_owned()
        }
    }

    fn symlink(&self, recipe_root: &PathBuf, source: &PathBuf, to: &PathBuf) -> Vec<Outcome> {
        let mut outcomes = vec![];

        let source_path = recipe_root.join(source);

        if !source_path.exists() {
            outcomes.push(Outcome::Failure(
                format!("link: {}", source_path.display()),
                format!("source path '{}' does not exist.", source_path.display()),
            ));
        }

        let to_path = self.resolve_path(to);

        if to_path.exists() {
            outcomes.push(Outcome::Failure(
                format!("link: {}", source_path.display()),
                format!("to path '{}' already exists", to_path.display()),
            ));
        }

        if outcomes.is_empty() {
            println!(
                "// TODO: implement symlink from '{}' to '{}'",
                source_path.display(),
                to_path.display()
            );
        }

        outcomes
    }
}
