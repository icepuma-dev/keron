use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::{
    dry_run_or_failure, dry_run_or_success,
    model::{Link, Outcome},
};

pub(crate) struct LinkProcessor;

impl LinkProcessor {
    pub(crate) fn new() -> LinkProcessor {
        LinkProcessor {}
    }

    pub(crate) fn process(
        &self,
        approve: bool,
        recipe_root: &Path,
        link: &IndexMap<PathBuf, Link>,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        for (source, link) in link {
            outcomes.extend(self.symlink(approve, recipe_root, source, &link.to));
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

    fn symlink(
        &self,
        approve: bool,
        recipe_root: &Path,
        source: &PathBuf,
        to: &PathBuf,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        let source_path = recipe_root.join(source);

        if !source_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("link: {}", source_path.display()),
                format!("source path '{}' does not exist.", source_path.display())
            ));
        }

        let to_path = self.resolve_path(to);

        if to_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("link: {}", source_path.display()),
                format!("to path '{}' already exists", to_path.display())
            ));
        }

        if outcomes.is_empty() {
            if approve {
                #[cfg(windows)]
                match std::os::windows::fs::symlink_file(&source_path, &to_path) {
                    Ok(_) => {
                        outcomes.push(dry_run_or_success!(
                            approve,
                            format!("link: {}", source_path.display()),
                            format!("to: {}", to_path.display())
                        ));
                    }
                    Err(err) => {
                        outcomes.push(dry_run_or_failure!(
                            approve,
                            format!("link: {}", source_path.display()),
                            format!("to '{}' failed: {err}", to_path.display())
                        ));
                    }
                }

                #[cfg(unix)]
                match std::os::unix::fs::symlink(&source_path, &to_path) {
                    Ok(_) => {
                        outcomes.push(dry_run_or_success!(
                            approve,
                            format!("link: {}", source_path.display()),
                            format!("to: {}", to_path.display())
                        ));
                    }
                    Err(err) => {
                        outcomes.push(dry_run_or_failure!(
                            approve,
                            format!("link: {}", source_path.display()),
                            format!("to '{}' failed: {err}", to_path.display())
                        ));
                    }
                }
            } else {
                println!(
                    "dry-run: link '{}' to '{}'",
                    source_path.display(),
                    to_path.display()
                );
            }
        }

        outcomes
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::model::Outcome;

    use super::LinkProcessor;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_symlink() {
        let link_processor = LinkProcessor::new();

        let recipe_root = tempfile::tempdir().unwrap();

        let mut npmrc = NamedTempFile::new_in(&recipe_root).unwrap();
        write!(npmrc, "fkbr").unwrap();

        let source = npmrc.path().to_path_buf();

        let target_directory = tempfile::tempdir().unwrap();
        let to = target_directory.path().join(".npmrc").to_path_buf();

        let outcomes = link_processor.symlink(true, &recipe_root.into_path(), &source, &to);

        assert_eq!(to.exists(), true);
        assert_eq!(outcomes.len(), 1);

        if let Outcome::Failure(_, _) = outcomes.get(0).unwrap() {
            panic!("the outcome should be successful!");
        }
    }
}
