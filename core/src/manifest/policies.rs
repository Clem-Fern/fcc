use serde::Deserialize;

use super::misc::FilesFromPath;

#[derive(Debug, Clone, Deserialize)]
pub struct Policies {
    #[serde(flatten)]
    paths: FilesFromPath,
}
