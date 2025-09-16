use std::error;
use std::fmt;

use crate::compliance::options::ComplianceOptionsError;

#[derive(Debug)]
pub enum ParseError {
    BadIndentation(String),
    OrphanComplianceOption,
    ComplianceOption(ComplianceOptionsError),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BadIndentation(ref line) => {
                write!(f, "Indentation incoherence line: \"{line}\"")
            }
            Self::ComplianceOption(ref err) => {
                write!(f, "{err}")
            }
            Self::OrphanComplianceOption => {
                write!(
                    f,
                    "Orphan compliance options found without associated content.\""
                )
            }
        }
    }
}

impl From<ComplianceOptionsError> for ParseError {
    fn from(err: ComplianceOptionsError) -> Self {
        Self::ComplianceOption(err)
    }
}
