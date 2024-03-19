use std::{collections::BTreeMap, fs, path::PathBuf};

use walkdir::WalkDir;

use crate::model::Recipe;

/// walks through a given folder and yields all [`Recipe`]s.
///
/// Max traversal depth is `3`.
pub(crate) fn load_all_recipes(root: &PathBuf) -> anyhow::Result<BTreeMap<String, Recipe>> {
    let mut recipes = BTreeMap::<String, Recipe>::new();

    for entry in WalkDir::new(root)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .map(|extenstion| extenstion == "hcl")
                .unwrap_or(false)
        {
            match fs::read_to_string(path) {
                Ok(file_content) => match hcl::from_str(&file_content) {
                    Ok(recipe) => {
                        match path.file_name() {
                            Some(file_name) => {
                                // FIXME: try to avoid lossy
                                recipes.insert(file_name.to_string_lossy().to_string(), recipe);
                            }
                            None => eprintln!(
                                "Cannot extract filename without extension from file '{}'",
                                path.display()
                            ),
                        }
                    }
                    Err(err) => {
                        eprintln!("Cannot parse file '{}' to a recipe: {err}", path.display())
                    }
                },
                Err(err) => eprintln!("Cannot read content from file '{}': {err}", path.display()),
            }
        }
    }

    Ok(recipes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::File, io::Write};
    use tempfile::TempDir;

    fn write_recipe(content: &str, file_name: &str, path: &TempDir) {
        let recipe_path = path.path().join(file_name);
        let mut recipe = File::create(recipe_path).unwrap();

        write!(recipe, "{content}").unwrap();
    }

    #[test]
    fn test_load_all_recipes() {
        let root = tempfile::tempdir().unwrap();

        let first_recipe = indoc! {r#"
            link ".npmrc" {
                to = "~/.npmrc"
            }
        "#};

        write_recipe(first_recipe, "first_recipe.hcl", &root);

        let subfolder = tempfile::tempdir_in(&root).unwrap();

        write_recipe("", "empty_recipe.hcl", &subfolder);

        assert_eq!(load_all_recipes(&root.into_path()).unwrap().len(), 2);
    }
}
