pub(crate) mod parse;

use std::{error, fmt};

use parse::parse_raw_options;
use regex::Error as RegexError;
use strum::EnumString;

use crate::parse::options::ComplianceOption;

#[derive(Debug)]
pub enum ComplianceOptionsError {
    BadIndentation(String),
    UnknowOption(String),
    MalformedOption(String),
    DuplicatedOption(ComplianceOption),
    InvalidOptionArgument(String, String),
    InvalidRegex(RegexError, String),
}

impl error::Error for ComplianceOptionsError {}

impl fmt::Display for ComplianceOptionsError {
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
            Self::DuplicatedOption(ref option) => {
                write!(f, "Option \"{option}\" specified more than one.")
            }
            Self::InvalidOptionArgument(ref arg, ref option) => {
                write!(
                    f,
                    "Unable to parse option argument \"{arg}\" from \"{option}\""
                )
            }
            Self::InvalidRegex(ref err, ref key) => {
                write!(f, "Regex error at line {key}. {err}")
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct ComplianceOptionsBuilder {
    pub(crate) regex: Option<bool>,
    pub(crate) state: Option<StateOption>,
    pub(crate) r#match: Option<MatchOption>,
}

impl ComplianceOptionsBuilder {
    pub fn regex(&mut self, regex: bool) -> Result<(), ComplianceOptionsError> {
        if self.regex.is_some() {
            return Err(ComplianceOptionsError::DuplicatedOption(
                ComplianceOption::Regex,
            ));
        }

        self.regex = Some(regex);
        Ok(())
    }

    pub fn state(&mut self, state: StateOption) -> Result<(), ComplianceOptionsError> {
        if self.state.is_some() {
            return Err(ComplianceOptionsError::DuplicatedOption(
                ComplianceOption::State(state),
            ));
        }

        self.state = Some(state);
        Ok(())
    }

    pub fn r#match(&mut self, r#match: MatchOption) -> Result<(), ComplianceOptionsError> {
        if self.r#match.is_some() {
            return Err(ComplianceOptionsError::DuplicatedOption(
                ComplianceOption::Match(r#match),
            ));
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComplianceOptions {
    pub(crate) regex: bool,
    pub(crate) state: StateOption,
    pub(crate) r#match: MatchOption,
}

impl ComplianceOptions {
    pub(crate) fn builder() -> ComplianceOptionsBuilder {
        ComplianceOptionsBuilder::default()
    }

    pub(crate) fn new_from_vec(options: &[String]) -> Result<Self, ComplianceOptionsError> {
        let mut compliance_option_builder = Self::builder();

        parse_raw_options(&mut compliance_option_builder, options)?;

        Ok(compliance_option_builder.build())
    }

    pub(crate) fn new_from_options_vec(
        options: &[ComplianceOption],
    ) -> Result<Self, ComplianceOptionsError> {
        let mut compliance_option_builder = Self::builder();
        for option in options {
            match option {
                ComplianceOption::Regex => compliance_option_builder.regex(true)?,
                ComplianceOption::State(state) => compliance_option_builder.state(*state)?,
                ComplianceOption::Match(r#match) => compliance_option_builder.r#match(*r#match)?,
            }
        }
        Ok(compliance_option_builder.build())
    }
}

impl Default for ComplianceOptions {
    fn default() -> Self {
        ComplianceOptionsBuilder::default().build()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq, Eq, PartialOrd, Ord)]
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
    use crate::parse::options::ComplianceOption;

    use super::ComplianceOptions;
    use super::ComplianceOptionsError;
    use super::MatchOption;
    use super::StateOption;

    #[test]
    fn test_compliance_options_default() {
        let options = ComplianceOptions::default();

        assert!(!options.regex);
        assert!(matches!(options.state, StateOption::Present));
    }

    #[test]
    fn test_compliance_options_builder() {
        let regex_options = [false, true];
        let state_options = [
            StateOption::Present,
            StateOption::Optional,
            StateOption::Absent,
        ];
        let match_options = [MatchOption::First, MatchOption::All];

        for &regex in &regex_options {
            for &state in &state_options {
                for &r#match in &match_options {
                    let mut options = Vec::new();
                    if regex {
                        options.push(ComplianceOption::Regex);
                    }
                    options.push(ComplianceOption::State(state));
                    options.push(ComplianceOption::Match(r#match));

                    let result = ComplianceOptions::new_from_options_vec(&options);
                    assert!(
                        result.is_ok(),
                        "Failed for combination: regex={:?}, state={:?}, match={:?}",
                        regex,
                        state,
                        r#match
                    );

                    let opts = result.unwrap();
                    assert_eq!(
                        opts.regex, regex,
                        "Regex mismatch for combination: {:?}",
                        options
                    );
                    assert_eq!(
                        opts.state, state,
                        "State mismatch for combination: {:?}",
                        options
                    );
                    assert_eq!(
                        opts.r#match, r#match,
                        "Match mismatch for combination: {:?}",
                        options
                    );
                }
            }
        }
    }

    #[test]
    fn test_compliance_options_builder_default() {
        let options = ComplianceOptions::new_from_options_vec(&vec![
            ComplianceOption::Regex,
            ComplianceOption::Match(MatchOption::All),
        ]);

        assert!(options.is_ok());
        let options = options.unwrap();

        assert!(options.regex);
        assert!(matches!(options.r#match, MatchOption::All));

        let options = ComplianceOptions::new_from_options_vec(&vec![ComplianceOption::State(
            StateOption::Absent,
        )]);

        assert!(options.is_ok());
        let options = options.unwrap();
        assert!(matches!(options.state, StateOption::Absent));
        assert!(matches!(options.r#match, MatchOption::All));
    }

    #[test]
    fn test_compliance_options_duplicated() {
        for option in &[
            ComplianceOption::Regex,
            ComplianceOption::State(StateOption::Present),
            ComplianceOption::Match(MatchOption::First),
        ] {
            let options = ComplianceOptions::new_from_options_vec(&vec![*option, *option]);

            assert!(matches!(
                options.err(),
                Some(ComplianceOptionsError::DuplicatedOption(_))
            ));
        }
    }
}
