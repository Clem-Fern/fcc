pub mod misc;
pub mod validate;
pub mod configurations;

use configurations::Configurations;
use misc::FilesFromPath;
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

// impl ComplianceManifest {
//     pub fn test(&self) {
//         validate_configs_1vec_1rm(deserializer)
//     }
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Policies {
    #[serde(flatten)]
    paths: FilesFromPath,
}

#[cfg(test)]
mod tests {
    use crate::manifest::ComplianceManifest;

    #[test]
    fn test_compliance_manifest() {
        let decoded: ComplianceManifest =
            toml::from_str(include_str!("../../test/manifest1.toml")).unwrap();
        println!("{decoded:#?}");
        assert!(true)
    }
}
