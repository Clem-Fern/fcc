use std::error;
use std::fmt;
use std::io;

use toml::de::Error as ParseError;

#[derive(Debug)]
pub enum ManifestError {
    Parse(ParseError),
    IO(io::Error),
}

impl error::Error for ManifestError {}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => {
                write!(f, "Compliance manifest error: {err}")
            }
            Self::IO(ref err) => {
                write!(f, "Compliance manifest IO error: {err}")
            }
        }
    }
}

impl From<ParseError> for ManifestError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}
