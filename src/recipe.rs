use std::{
    fs,
    path::{Path, PathBuf},
};

use glob::glob;

#[derive(Debug, Clone)]
pub(crate) struct Recipe {
    pub(crate) _path: PathBuf,
    pub(crate) content: String,
}

pub(crate) struct RecipeResolver;

impl RecipeResolver {
    pub(crate) fn new() -> Self {
        RecipeResolver {}
    }

    pub(crate) fn resolve(&self, recipe_root: &Path) -> anyhow::Result<Vec<Recipe>> {
        let absolute_path = recipe_root.canonicalize()?;

        let mut recipes = vec![];

        for path in glob(&format!("{}/**/*.keron", absolute_path.display()))? {
            match path {
                Ok(path) => recipes.push(self.to_recipe(&path)?),
                Err(_) => todo!("..."),
            }
        }

        Ok(recipes)
    }

    fn to_recipe(&self, path: &Path) -> anyhow::Result<Recipe> {
        let content = fs::read_to_string(path)?;

        Ok(Recipe {
            _path: path.to_path_buf(),
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RecipeResolver;
    use std::io::Write;
    use tempfile::{tempdir, tempdir_in, NamedTempFile};

    #[test]
    fn resolve_recipes() {
        let resolver = RecipeResolver::new();
        let recipe_root = tempdir().unwrap();
        let mut first_recipe = NamedTempFile::with_suffix_in(".keron", &recipe_root).unwrap();
        writeln!(first_recipe, "print(\"first\")").unwrap();

        let nested_recipe_folder = tempdir_in(&recipe_root).unwrap();

        let mut second_recipe =
            NamedTempFile::with_suffix_in(".keron", &nested_recipe_folder).unwrap();
        writeln!(second_recipe, "print(\"second\")").unwrap();

        let result = resolver.resolve(&recipe_root.into_path()).unwrap();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn to_recipe() {
        let resolver = RecipeResolver::new();
        let mut recipe_file = NamedTempFile::with_suffix(".keron").unwrap();

        let content = "print(\"recipe\")".to_string();
        writeln!(recipe_file, "{content}").unwrap();

        let recipe = resolver.to_recipe(recipe_file.path()).unwrap();

        assert_eq!(recipe._path, recipe_file.path());
        assert_eq!(recipe.content, format!("{content}\n"));
    }
}
