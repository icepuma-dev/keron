use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Link {
    pub(crate) to: PathBuf,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) privileged: Option<bool>,
}
