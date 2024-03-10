pub(crate) mod link;
pub(crate) mod outcome;
pub(crate) mod package;
pub(crate) mod recipe;

pub(crate) use link::*;
pub(crate) use outcome::*;
pub(crate) use package::*;
pub(crate) use recipe::*;

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, path::PathBuf};

    use indexmap::indexmap;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use crate::model::Link;

    use super::{Packages, Recipe};

    fn assert_serialize<T: Serialize>(value: T, expected: &str) {
        assert_eq!(hcl::to_string(&value).unwrap(), expected);
    }

    fn assert_deserialize<'de, T>(input: &'de str, expected: T)
    where
        T: Deserialize<'de> + Debug + PartialEq,
    {
        assert_eq!(hcl::from_str::<T>(input).unwrap(), expected);
    }

    #[test]
    fn test_serialize() {
        let link = indexmap! {
            PathBuf::from(".npmrc") => Link { to: PathBuf::from("~/.npmrc") }
        };

        let packages = vec![Packages {
            list: Some(vec!["fzf".to_string()]),
            manager: None,
        }];

        let recipe = Recipe {
            link: Some(link),
            packages: Some(packages),
        };

        let expected = indoc! {r#"
            link ".npmrc" {
              to = "~/.npmrc"
            }

            packages {
              list = [
                "fzf"
              ]
            }
        "#};

        assert_serialize(recipe, expected);
    }

    #[test]
    fn test_deserialize() {
        let recipe = Recipe {
            link: Some(indexmap! {
                PathBuf::from(".npmrc") => Link { to: PathBuf::from("~/.npmrc") }
            }),
            packages: None,
        };

        let manifest_str = indoc! {r#"
            link ".npmrc" {
              to = "~/.npmrc"
            }
        "#};

        assert_deserialize(manifest_str, recipe)
    }
}
