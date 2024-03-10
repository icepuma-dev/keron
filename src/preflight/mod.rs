use std::collections::{BTreeMap, HashSet};

use which::which;

use crate::model::{PackageManager, Recipe};

pub(crate) fn preflight(recipes: &BTreeMap<String, Recipe>) {
    let mut package_managers = HashSet::<PackageManager>::new();

    for recipe in recipes.values() {
        recipe.packages.as_ref().iter().for_each(|packages| {
            packages.iter().for_each(|packages| {
                packages
                    .manager
                    .as_ref()
                    .unwrap_or(&HashSet::<PackageManager>::new())
                    .iter()
                    .for_each(|manager| {
                        package_managers.insert(manager.clone());
                    })
            })
        });
    }

    for package_manager in package_managers {
        match package_manager {
            PackageManager::Brew => match which("brew") {
                Ok(_) => println!("✅ brew is installed"),
                Err(_) => println!("❌ brew is not installed"),
            },
            PackageManager::Yay => match which("yay") {
                Ok(_) => println!("✅ yay is installed"),
                Err(_) => println!("❌ yay is not installed"),
            },
        }
    }
}
