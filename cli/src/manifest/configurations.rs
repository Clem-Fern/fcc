use std::fs::read_to_string;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use fcc::FlatConfig;
use regex::Regex;
use serde::Deserialize;

use super::error::ManifestError;
use super::misc::FilesFromPath;
use super::validate::validate_socket_addr_1vec;

#[derive(Debug, Clone, Deserialize)]
pub struct Configurations {
    #[serde(default)]
    #[serde(with = "serde_regex")]
    regignore: Option<Regex>,

    #[serde(default)]
    pub(crate) files: Vec<FilesFromPath>,

    #[serde(default)]
    #[serde(rename = "ssh-exec")]
    pub(crate) remote_ssh_exec: Vec<SshExecConfiguration>,
}

impl Configurations {
    pub fn get_configurations<P: AsRef<Path>>(
        &self,
        manifest_path: P,
    ) -> Vec<Result<FlatConfig, ManifestError>> {
        let mut vec = vec![];

        for files in &self.files {
            for path in files.get_files(&manifest_path) {
                match path {
                    Ok(path) => vec.push(config_from_path(path)),
                    Err(err) => vec.push(Err(err)),
                }
            }
        }

        // TODO
        // for remote in &self.remote_ssh_exec {
        //     let mut rconfigs = remote.retrieve_configurations()?;
        //     vec.append(&mut rconfigs);
        // }

        vec
    }
}

pub fn config_from_path(path: PathBuf) -> Result<FlatConfig, ManifestError> {
    let raw = read_to_string(&path).map_err(|err| ManifestError::IO(path.to_path_buf(), err))?;
    FlatConfig::from_str(&raw).map_err(|err| ManifestError::FlatConfig(path.to_path_buf(), err))
}

#[derive(Debug, Clone, Deserialize)]
pub struct SshExecConfiguration {
    cmd: String,
    user: String,
    #[serde(deserialize_with = "validate_socket_addr_1vec")]
    hosts: Vec<SocketAddr>,
}

impl SshExecConfiguration {
    pub fn retrieve_configurations(&self) -> Result<Vec<FlatConfig>, ManifestError> {
        todo!()
    }
}
