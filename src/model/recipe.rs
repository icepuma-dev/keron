use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Link;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Recipe {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub(crate) link: Option<IndexMap<PathBuf, Link>>,
}
