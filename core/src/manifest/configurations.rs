use std::net::SocketAddr;

use regex::Regex;
use serde::Deserialize;

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

#[derive(Debug, Clone, Deserialize)]
pub struct SshExecConfiguration {
    cmd: String,
    user: String,
    #[serde(deserialize_with = "validate_socket_addr_1vec")]
    hosts: Vec<SocketAddr>,
}
