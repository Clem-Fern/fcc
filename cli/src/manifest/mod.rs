pub mod configurations;
pub mod error;
pub mod misc;
pub mod policies;
pub mod validate;

use std::{path::Path, str::FromStr};

use configurations::Configurations;
use error::ManifestError;
use fcc::{FlatConfig, FlatConfigCompliance};
use policies::Policies;
use serde::Deserialize;
use validate::{validate_configs_1vec_1rm, validate_policies_1vec};

#[derive(Debug, Clone, Deserialize)]
pub struct ComplianceManifest {
    #[serde(deserialize_with = "validate_policies_1vec")]
    policies: Vec<Policies>,

    #[serde(rename = "configs")]
    #[serde(deserialize_with = "validate_configs_1vec_1rm")]
    configurations: Vec<Configurations>,
}

impl ComplianceManifest {
    pub fn get_policies<P: AsRef<Path>>(
        &self,
        manifest_path: P,
    ) -> Result<Vec<FlatConfigCompliance>, ManifestError> {
        let mut vec = vec![];

        for policy in &self.policies {
            let mut fccs = policy.get_policies(&manifest_path)?;
            vec.append(&mut fccs);
        }

        Ok(vec)
    }

    pub fn get_configurations<P: AsRef<Path>>(
        &self,
        manifest_path: P,
    ) -> Vec<Result<FlatConfig, ManifestError>> {
        let mut vec = vec![];

        for config in &self.configurations {
            let mut fcs = config.get_configurations(&manifest_path);
            vec.append(&mut fcs);
        }

        vec
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
        let decoded = ComplianceManifest::from_str(include_str!("../../test/manifest1.toml"));
        // println!("{decoded:#?}");
        assert!(decoded.is_ok());
    }

    #[test]
    fn test_compliance_manifest2() {
        let decoded = ComplianceManifest::from_str(include_str!("../../test/manifest2.toml"));
        // println!("{decoded:#?}");
        assert!(decoded.is_ok());

        let manifest = decoded.unwrap();
        let r = manifest.get_policies("./test/manifest2.toml");
        println!("{r:#?}");
        assert!(r.is_ok());
        println!("Policies found: {}", r.unwrap().len());

        let r = manifest.get_configurations("./test/manifest2.toml");
        println!("{r:#?}");
        println!("Configurations found: {}", r.len());
    }
}
