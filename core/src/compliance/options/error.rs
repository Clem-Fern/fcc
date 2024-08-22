use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    BadIndentation(String),
    UnknowOption(String),
    MalformedOption(String),
    InvalidOptionArgument(String, String),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::BadIndentation(ref line) => {
                write!(f, "Option indentation incoherence line: \"{line}\"")
            }
            Self::UnknowOption(ref option) => {
                write!(f, "Unable to parse unknow option: \"{option}\"")
            }
            Self::MalformedOption(ref option) => {
                write!(
                    f,
                    "Unable to parse malformed option and argument: \"{option}\""
                )
            }
            Self::InvalidOptionArgument(ref arg, ref option) => {
                write!(
                    f,
                    "Unable to parse option argument \"{arg}\" from \"{option}\""
                )
            }
        }
    }
}