use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::Link;

/// A recipe holds all elements which should be applied when running keron.
///
/// Supported elements:
/// * [`Link`] - "symlink" a source to a target
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Recipe {
    #[serde(
        serialize_with = "hcl::ser::labeled_block",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) link: Option<IndexMap<PathBuf, Link>>,
}
