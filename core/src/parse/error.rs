use std::error;
use std::fmt;

use crate::compliance::options::error::ParseError as ComplianceOptionParseError;

#[derive(Debug)]
pub enum ParseError {
    BadIndentation(String),
    ComplianceOption(ComplianceOptionParseError),
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
        }
    }
}

impl From<ComplianceOptionParseError> for ParseError {
    fn from(err: ComplianceOptionParseError) -> Self {
        Self::ComplianceOption(err)
    }
}
