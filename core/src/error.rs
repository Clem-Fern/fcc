use std::error;
use std::fmt;
use std::io;

pub use crate::compliance::options::error::ParseError as ComplianceOptionParseError;
pub use crate::parse::error::ParseError;

#[derive(Debug)]
pub enum FlatConfigError {
    Parse(ParseError),
    IO(io::Error),
}

impl error::Error for FlatConfigError {}

impl fmt::Display for FlatConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => {
                write!(f, "Flat configuration parsing error: {err}")
            }
            Self::IO(ref err) => {
                write!(f, "Flat configuration IO error: {err}")
            }
        }
    }
}

impl From<ParseError> for FlatConfigError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl From<ComplianceOptionParseError> for FlatConfigError {
    fn from(err: ComplianceOptionParseError) -> Self {
        Self::Parse(err.into())
    }
}
