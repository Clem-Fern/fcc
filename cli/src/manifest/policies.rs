use std::{fs::read_to_string, path::Path, str::FromStr};

use fcc::FlatConfigCompliance;
use serde::Deserialize;

use super::{error::ManifestError, misc::FilesFromPath};

#[derive(Debug, Clone, Deserialize)]
pub struct Policies {
    #[serde(flatten)]
    paths: FilesFromPath,
}

impl Policies {
    pub fn get_policies<P: AsRef<Path>>(
        &self,
        manifest_path: P,
    ) -> Result<Vec<FlatConfigCompliance>, ManifestError> {
        let mut vec = vec![];

        let paths = self
            .paths
            .get_files(&manifest_path)
            .into_iter()
            .collect::<Result<Vec<_>, ManifestError>>()?;
        for path in paths {
            let raw =
                read_to_string(&path).map_err(|err| ManifestError::IO(path.to_path_buf(), err))?;
            vec.push(
                FlatConfigCompliance::from_str(&raw)
                    .map_err(|err| ManifestError::FlatConfig(path.to_path_buf(), err))?,
            );
        }

        Ok(vec)
    }
}
