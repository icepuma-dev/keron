use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::{Link, Packages};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Recipe {
    #[serde(
        serialize_with = "hcl::ser::labeled_block",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) link: Option<IndexMap<PathBuf, Link>>,

    #[serde(
        serialize_with = "hcl::ser::block",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) packages: Option<Vec<Packages>>,
}
