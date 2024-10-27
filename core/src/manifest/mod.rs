pub mod configurations;
pub mod error;
pub mod misc;
pub mod policies;
pub mod validate;

use std::{path::PathBuf, str::FromStr};

use configurations::Configurations;
use error::ManifestError;
use policies::Policies;
use serde::Deserialize;
use validate::{validate_configs_1vec_1rm, validate_policies_1vec};

use crate::{FlatConfig, FlatConfigCompliance};

#[derive(Debug, Clone, Deserialize)]
pub struct ComplianceManifest {
    #[serde(deserialize_with = "validate_policies_1vec")]
    policies: Vec<Policies>,

    #[serde(rename = "configs")]
    #[serde(deserialize_with = "validate_configs_1vec_1rm")]
    configurations: Vec<Configurations>,
}

impl ComplianceManifest {
    pub fn get_policies(
        &self,
        manifest_path: PathBuf,
    ) -> Result<Vec<FlatConfigCompliance>, ManifestError> {
        todo!()
    }

    pub fn get_configurations(
        &self,
        manifest_path: PathBuf,
    ) -> Result<Vec<FlatConfig>, ManifestError> {
        todo!()
    }
}

impl FromStr for ComplianceManifest {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str::<Self>(s)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::manifest::ComplianceManifest;
    use std::str::FromStr;

    #[test]
    fn test_compliance_manifest() {
        let decoded =
            ComplianceManifest::from_str(include_str!("../../test/manifest1.toml")).unwrap();
        println!("{decoded:#?}");
        assert!(true)
    }
}
