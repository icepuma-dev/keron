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
        recipe_name: &String,
        recipe_root: &Path,
        link: &IndexMap<PathBuf, Link>,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        for (source, link) in link {
            outcomes.extend(self.symlink(approve, recipe_name, recipe_root, source, &link.to));
        }

        outcomes
    }

    fn symlink(
        &self,
        approve: bool,
        recipe_name: &String,
        recipe_root: &Path,
        source: &PathBuf,
        to: &Path,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        let source_path = recipe_root.join(source);

        if !source_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("{recipe_name}/link/{}", source.display()),
                format!("link {} to {}", source.display(), to.display())
            ));
        }

        let to_path = PathBuf::from(shellexpand::tilde(&to.display().to_string()).to_string());

        if to_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("{recipe_name}/link/{}", source.display()),
                format!("source '{}' already exists", source.display())
            ));
        }

        if outcomes.is_empty() {
            if approve {
                #[cfg(windows)]
                match std::os::windows::fs::symlink_file(&source_path, &to_path) {
                    Ok(_) => {
                        outcomes.push(dry_run_or_success!(approve,));
                    }
                    Err(err) => {
                        outcomes.push(dry_run_or_failure!(
                            approve,
                            format!("{recipe_name}/link/{}", source.display()),
                            format!("to path '{}' already exists", to_path.display())
                        ));
                    }
                }

                #[cfg(unix)]
                match std::os::unix::fs::symlink(&source_path, &to_path) {
                    Ok(_) => {
                        outcomes.push(dry_run_or_success!(
                            approve,
                            format!("{recipe_name}/link/{}", source.display()),
                            format!(
                                "successfully linked '{}' to '{}'",
                                source.display(),
                                to.display()
                            )
                        ));
                    }
                    Err(err) => {
                        outcomes.push(dry_run_or_failure!(
                            approve,
                            format!("{recipe_name}/link/{}", source.display()),
                            format!(
                                "link from '{}' to '{}' failed: {err}",
                                source.display(),
                                to.display()
                            )
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

        let outcomes = link_processor.symlink(
            true,
            &"fkbr".to_string(),
            &recipe_root.into_path(),
            &source,
            &to,
        );

        assert_eq!(to.exists(), true);
        assert_eq!(outcomes.len(), 1);

        if let Outcome::Failure { .. } = outcomes.first().unwrap() {
            panic!("the outcome should be successful!");
        }
    }
}
