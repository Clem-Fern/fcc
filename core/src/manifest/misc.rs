use std::path::PathBuf;

use serde::Deserialize;
use super::validate::validate_path_buf_1vec;

#[derive(Debug, Clone, Deserialize)]
pub struct FilesFromPath {
    #[serde(default)]
    recurse: bool,
    #[serde(deserialize_with = "validate_path_buf_1vec")]
    paths: Vec<PathBuf>,
}

