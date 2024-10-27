use std::error;
use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;

use super::configurations::Configurations;
use super::Policies;
use serde::{de, Deserializer};

#[derive(Debug)]
pub enum ValidateError {
    Vec1Policies,
    Vec1Configurations,
    Vec1PathBuf,
    Vec1SocketAddr,
    Rm1Configurations,
}

impl error::Error for ValidateError {}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ValidateError::Vec1Policies => {
                write!(f, "At least one way to retrieve policies must be present.")
            }
            ValidateError::Vec1Configurations => write!(
                f,
                "At least one way to retrieve configurations must be present."
            ),
            ValidateError::Vec1PathBuf => write!(f, "\"paths\" must contain at least one element."),
            ValidateError::Vec1SocketAddr => {
                write!(f, "\"hosts\" must contain at least one element.")
            }
            ValidateError::Rm1Configurations => write!(
                f,
                "\"configs\" must contain at least one valid method to retrieve configurations."
            ),
        }
    }
}

pub(crate) fn validate_policies_1vec<'de, D>(deserializer: D) -> Result<Vec<Policies>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<Policies> = de::Deserialize::deserialize(deserializer)?;
    /* identical to check_vec_not_empty body */
    if v.is_empty() {
        Err(de::Error::custom(ValidateError::Vec1Policies))
    } else {
        Ok(v)
    }
}

pub(crate) fn validate_configs_1vec_1rm<'de, D>(
    deserializer: D,
) -> Result<Vec<Configurations>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<Configurations> = de::Deserialize::deserialize(deserializer)?;
    /* identical to check_vec_not_empty body */
    if v.is_empty() {
        Err(de::Error::custom(ValidateError::Vec1Configurations))
    } else {
        for c in v.clone() {
            if c.files.is_empty() && c.remote_ssh_exec.is_empty() {
                return Err(de::Error::custom(ValidateError::Rm1Configurations));
            }
        }
        Ok(v)
    }
}

pub(crate) fn validate_path_buf_1vec<'de, D>(deserializer: D) -> Result<Vec<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<PathBuf> = de::Deserialize::deserialize(deserializer)?;
    /* identical to check_vec_not_empty body */
    if v.is_empty() {
        Err(de::Error::custom(ValidateError::Vec1PathBuf))
    } else {
        Ok(v)
    }
}

pub(crate) fn validate_socket_addr_1vec<'de, D>(
    deserializer: D,
) -> Result<Vec<SocketAddr>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<SocketAddr> = de::Deserialize::deserialize(deserializer)?;
    /* identical to check_vec_not_empty body */
    if v.is_empty() {
        Err(de::Error::custom(ValidateError::Vec1SocketAddr))
    } else {
        Ok(v)
    }
}
