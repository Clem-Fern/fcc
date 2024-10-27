use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

use fcc::error::FlatConfigError;
use toml::de::Error as ParseError;

#[derive(Debug)]
pub enum ManifestError {
    Parse(ParseError),
    IO(PathBuf, io::Error),
    FlatConfig(PathBuf, FlatConfigError),
}

impl error::Error for ManifestError {}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Parse(ref err) => write!(f, "Compliance manifest error: {err}"),
            Self::IO(ref path, ref err) => write!(f, "{}: {err}", path.display()),
            Self::FlatConfig(ref path, ref err) => write!(f, "{}: {err}", path.display()),
        }
    }
}

impl From<ParseError> for ManifestError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}
