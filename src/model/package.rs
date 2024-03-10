use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Package {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) manager: Option<HashSet<PackageManager>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub(crate) enum PackageManager {
    #[serde(rename = "brew")]
    Brew,

    #[serde(rename = "yay")]
    Yay,
}
