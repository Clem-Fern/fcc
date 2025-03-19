pub mod error;
pub(crate) mod parse;

use std::fmt;

use error::ParseError;
use parse::parse_raw_options;
use strum::EnumString;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct ComplianceOptions {
    pub(crate) regex: bool,
    pub(crate) match_type: MatchOption,
}

impl ComplianceOptions {
    pub(crate) fn new_from_vec(options: &[String]) -> Result<Self, ParseError> {
        let mut compliance_option = Self::default();

        parse_raw_options(&mut compliance_option, options)?;

        Ok(compliance_option)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum MatchOption {
    #[default]
    Present,
    Optional,
    Absent,
}

impl fmt::Display for MatchOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait ComplianceOptionsContainer {
    fn get_options(&self) -> ComplianceOptions;
    fn set_options(&mut self, options: ComplianceOptions);
    #[cfg(debug_assertions)]
    fn get_raw_options(&self) -> &Vec<String>;
    #[cfg(debug_assertions)]
    fn set_raw_options(&mut self, options: &[String]);
}

#[cfg(test)]
mod tests {
    use super::ComplianceOptions;
    use super::MatchOption;

    #[test]
    fn test_fcc_options_default() {
        let options = ComplianceOptions::default();

        assert_eq!(options.regex, false);
        assert!(matches!(options.match_type, MatchOption::Present));
    }
}
