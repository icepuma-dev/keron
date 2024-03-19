use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::{
    dry_run_or_failure, dry_run_or_success,
    model::{Link, Outcome},
};

/// Process [`Link`]s when applying a [`crate::model::Recipe`].
pub(crate) struct LinkProcessor;

/// Models the outcomes a symlink application can have:
/// * success - everything went smoothly
/// * requires elevation - running as a [`elevate::RunningAs::User`] and the [`Link`] is [`Link::privileged`]
/// * elevated but not wanted - running as a [`elevate::RunningAs::Root`] or [`elevate::RunningAs::Suid`] and the [`Link`] is not [`Link::privileged`]
enum SymlinkResult {
    Success,

    #[allow(dead_code)]
    RequiresElevation,

    #[allow(dead_code)]
    ElevatedButNotWanted,
}

impl LinkProcessor {
    /// Create new [`LinkProcessor`]
    pub(crate) fn new() -> LinkProcessor {
        LinkProcessor {}
    }

    /// Process a collection of [`Link`]s
    ///
    /// When [`LinkProcessor::process`] is called with `approve` = true we try to apply all links,
    /// otherwise we assume a "dry-run" and don't do anything
    ///
    /// A list of [`Outcome`]s is yielded after each run
    pub(crate) fn process(
        &self,
        approve: bool,
        recipe_name: &String,
        recipe_root: &Path,
        link: &IndexMap<PathBuf, Link>,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        for (source, link) in link {
            outcomes.extend(self.link(approve, recipe_name, recipe_root, source, link));
        }

        outcomes
    }

    /// Link a source to a target
    ///
    /// If a target contains a "~" it is expanded to [`dirs::home_dir()`].
    fn link(
        &self,
        approve: bool,
        recipe_name: &String,
        recipe_root: &Path,
        source: &PathBuf,
        link: &Link,
    ) -> Vec<Outcome> {
        let mut outcomes = vec![];

        let source_path = recipe_root.join(source);

        if !source_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("{recipe_name}/link/{}", source.display()),
                format!("link {} to {}", source.display(), link.to.display())
            ));
        }

        let to_path = PathBuf::from(shellexpand::tilde(&link.to.display().to_string()).to_string());

        if to_path.exists() {
            outcomes.push(dry_run_or_failure!(
                approve,
                format!("{recipe_name}/link/{}", source.display()),
                format!("source '{}' already exists", source.display())
            ));
        }

        let privileged = link.privileged.unwrap_or(false);

        if outcomes.is_empty() {
            if approve {
                match self.symlink(&source_path, &to_path, privileged) {
                    Ok(result) => match result {
                        SymlinkResult::Success => {
                            outcomes.push(dry_run_or_success!(
                                approve,
                                format!("{recipe_name}/link/{}", source.display()),
                                format!(
                                    "successfully linked '{}' to '{}'",
                                    source.display(),
                                    to_path.display()
                                )
                            ));
                        }
                        SymlinkResult::RequiresElevation => {
                            outcomes.push(dry_run_or_failure!(
                                approve,
                                format!("{recipe_name}/link/{}", source.display()),
                                format!("link requires elevation, rerun with sudo",)
                            ));
                        }
                        SymlinkResult::ElevatedButNotWanted => {
                            outcomes.push(dry_run_or_failure!(
                                approve,
                                format!("{recipe_name}/link/{}", source.display()),
                                format!("running elevated, but link does not require elevation",)
                            ));
                        }
                    },
                    Err(err) => {
                        outcomes.push(dry_run_or_failure!(
                            approve,
                            format!("{recipe_name}/link/{}", source.display()),
                            format!(
                                "link from '{}' to '{}' failed: {err}",
                                source.display(),
                                to_path.display()
                            )
                        ));
                    }
                }
            } else {
                let privileged_text = if privileged {
                    " - will require sudo / root to run"
                } else {
                    ""
                };

                println!(
                    "dry-run: link '{}' to '{}'{}",
                    source_path.display(),
                    to_path.display(),
                    privileged_text
                );
            }
        }

        outcomes
    }

    /// Symlink a source to a target under on Windows.
    ///
    /// Supports files and folder sources.
    #[cfg(windows)]
    fn symlink(
        &self,
        source_path: &PathBuf,
        to: &PathBuf,
        _privileged: bool,
    ) -> anyhow::Result<SymlinkResult> {
        if source_path.is_dir() {
            std::os::windows::fs::symlink_dir(source_path, to)
                .map_err(anyhow::Error::from)
                .map(|_| SymlinkResult::Success)
        } else {
            std::os::windows::fs::symlink_file(source_path, to)
                .map_err(anyhow::Error::from)
                .map(|_| SymlinkResult::Success)
        }
    }

    /// Symlink a source to a target on Linux, macOS and other Unix-like systems.
    ///
    /// Supports files and folder sources.
    ///
    /// When applying a symlink for a "privileged" [`Link`],
    /// we try to ensure that the link belongs to the user who elevated into "sudo".
    ///
    /// We use the env vars `SUDO_UID` and `SUDO_GID` as they're preserved when running in "sudo",
    /// the fallback is `uid = 0` and `gid = 0` which is "root".
    ///
    /// TODO: discuss if "root" is an adequate fallback
    #[cfg(unix)]
    fn symlink(
        &self,
        source_path: &PathBuf,
        to: &PathBuf,
        privileged: bool,
    ) -> anyhow::Result<SymlinkResult> {
        use std::env;

        use elevate::RunningAs;

        let running_as = elevate::check();

        if (running_as == RunningAs::Root || running_as == RunningAs::Suid) && !privileged {
            return Ok(SymlinkResult::ElevatedButNotWanted);
        } else if running_as == RunningAs::User && privileged {
            return Ok(SymlinkResult::RequiresElevation);
        }

        std::os::unix::fs::symlink(source_path, to)
            .map_err(anyhow::Error::from)
            .and_then(|_| {
                if privileged {
                    let real_user_uid = env::var("SUDO_UID")
                        .map_err(anyhow::Error::from)
                        .and_then(|var| var.parse::<u32>().map_err(anyhow::Error::from))
                        .unwrap_or(0_u32);

                    let real_user_gid = env::var("SUDO_GID")
                        .map_err(anyhow::Error::from)
                        .and_then(|var| var.parse::<u32>().map_err(anyhow::Error::from))
                        .unwrap_or(0_u32);

                    std::os::unix::fs::lchown(to, Some(real_user_uid), Some(real_user_gid))
                        .map(|_| SymlinkResult::Success)
                        .map_err(anyhow::Error::from)
                } else {
                    Ok(SymlinkResult::Success)
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::model::{Link, Outcome};

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

        let outcomes = link_processor.link(
            true,
            &"fkbr".to_string(),
            &recipe_root.into_path(),
            &source,
            &Link {
                to: to.clone(),
                privileged: None,
            },
        );

        assert_eq!(to.exists(), true);
        assert_eq!(outcomes.len(), 1);

        if let Outcome::Failure { .. } = outcomes.first().unwrap() {
            panic!("the outcome should be successful!");
        }
    }
}
