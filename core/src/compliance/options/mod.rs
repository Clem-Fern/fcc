pub mod error;
pub(crate) mod parse;

use std::fmt;

use error::ParseError;
use parse::parse_raw_options;
use strum::EnumString;

#[derive(Debug, Default, Copy, Clone)]
pub struct ComplianceOptionsBuilder {
    pub(crate) regex: Option<bool>,
    pub(crate) state: Option<StateOption>,
    pub(crate) r#match: Option<MatchOption>,
}

impl ComplianceOptionsBuilder {
    pub fn regex(&mut self, regex: bool) -> Result<(), ParseError> {
        if self.regex.is_some() {
            return Err(ParseError::DuplicatedOption("regex".to_string()));
        }

        self.regex = Some(regex);
        Ok(())
    }

    pub fn state(&mut self, state: StateOption) -> Result<(), ParseError> {
        if self.state.is_some() {
            return Err(ParseError::DuplicatedOption("state".to_string()));
        }

        self.state = Some(state);
        Ok(())
    }

    pub fn r#match(&mut self, r#match: MatchOption) -> Result<(), ParseError> {
        if self.r#match.is_some() {
            return Err(ParseError::DuplicatedOption("match".to_string()));
        }

        self.r#match = Some(r#match);
        Ok(())
    }

    pub fn build(self) -> ComplianceOptions {
        let regex = self.regex.unwrap_or(false);
        let state = self.state.unwrap_or_default();
        ComplianceOptions {
            regex,
            state,
            r#match: self
                .r#match
                .unwrap_or(if regex | matches!(state, StateOption::Absent) {
                    MatchOption::All
                } else {
                    MatchOption::default()
                }),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ComplianceOptions {
    pub(crate) regex: bool,
    pub(crate) state: StateOption,
    pub(crate) r#match: MatchOption,
}

impl ComplianceOptions {
    pub(crate) fn builder() -> ComplianceOptionsBuilder {
        ComplianceOptionsBuilder::default()
    }

    pub(crate) fn new_from_vec(options: &[String]) -> Result<Self, ParseError> {
        let mut compliance_option_builder = Self::builder();

        parse_raw_options(&mut compliance_option_builder, options)?;

        Ok(compliance_option_builder.build())
    }
}

impl Default for ComplianceOptions {
    fn default() -> Self {
        ComplianceOptionsBuilder::default().build()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum MatchOption {
    #[default]
    First,
    All,
}

impl fmt::Display for MatchOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum StateOption {
    #[default]
    Present,
    Optional,
    Absent,
}

impl fmt::Display for StateOption {
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
    use super::StateOption;

    #[test]
    fn test_fcc_options_default() {
        let options = ComplianceOptions::default();

        assert!(!options.regex);
        assert!(matches!(options.state, StateOption::Present));
    }
}
