use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A link between a source and a target.
/// A "privileged" link is only handled when running as a root or with sudo.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Link {
    pub(crate) to: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) privileged: Option<bool>,
}
