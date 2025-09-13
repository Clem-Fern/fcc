use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        char,
        complete::{alpha1, newline, space0},
    },
    combinator::{cut, map, map_res},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use strum::Display;

use crate::compliance::options::{ComplianceOptions, MatchOption, StateOption};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum ComplianceOption {
    Regex,
    State(StateOption),
    Match(MatchOption),
}

pub fn compliance_options(input: &str) -> IResult<&str, ComplianceOptions> {
    many0(terminated(compliance_option, pair(space0, newline)))
        .map_res(|options| ComplianceOptions::new_from_options_vec(&options))
        .parse(input)
}

pub fn compliance_option(input: &str) -> IResult<&str, ComplianceOption> {
    delimited(
        tag("#["),
        cut(alt((
            map(tag("regex"), |_| ComplianceOption::Regex),
            map(
                preceded(tag("state="), state_option),
                ComplianceOption::State,
            ),
            map(
                preceded(tag("match="), match_option),
                ComplianceOption::Match,
            ),
        ))),
        char(']'),
    )
    .parse(input)
}

fn state_option(input: &str) -> IResult<&str, StateOption> {
    map_res(alpha1, |s: &str| StateOption::from_str(s)).parse(input)
}

fn match_option(input: &str) -> IResult<&str, MatchOption> {
    map_res(alpha1, |s: &str| MatchOption::from_str(s)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_match_option() {
        assert_eq!(match_option("all"), Ok(("", MatchOption::All)));
        assert_eq!(match_option("first"), Ok(("", MatchOption::First)));
        assert!(match_option("invalid").is_err());
    }

    #[test]
    fn test_parse_state_option() {
        assert_eq!(state_option("present"), Ok(("", StateOption::Present)));
        assert_eq!(state_option("optional"), Ok(("", StateOption::Optional)));
        assert_eq!(state_option("absent"), Ok(("", StateOption::Absent)));
        assert!(state_option("invalid").is_err());
    }

    #[test]
    fn test_parse_regex_compliance_option() {
        assert_eq!(
            compliance_option("#[regex]"),
            Ok(("", ComplianceOption::Regex))
        );
        assert!(compliance_option("#[regex=]").is_err());
    }

    #[test]
    fn test_parse_match_compliance_option() {
        assert_eq!(
            compliance_option("#[match=first]"),
            Ok(("", ComplianceOption::Match(MatchOption::First)))
        );
        assert_eq!(
            compliance_option("#[match=all]"),
            Ok(("", ComplianceOption::Match(MatchOption::All)))
        );
        assert!(compliance_option("#[match=invalid]").is_err());
    }

    #[test]
    fn test_parse_state_compliance_option() {
        assert_eq!(
            compliance_option("#[state=present]"),
            Ok(("", ComplianceOption::State(StateOption::Present)))
        );
        assert_eq!(
            compliance_option("#[state=optional]"),
            Ok(("", ComplianceOption::State(StateOption::Optional)))
        );
        assert_eq!(
            compliance_option("#[state=absent]"),
            Ok(("", ComplianceOption::State(StateOption::Absent)))
        );
        assert!(compliance_option("#[state=invalid]").is_err());
    }

    #[test]
    fn test_parse_compliance_options() {
        let input = "#[regex]\n#[state=present]\n#[match=all]\n";
        assert_eq!(
            compliance_options(input),
            Ok((
                "",
                ComplianceOptions {
                    regex: true,
                    state: StateOption::Present,
                    r#match: MatchOption::All
                }
            ))
        );

        let input = "#[regex]\t\n#[state=optional]   \n#[match=first] \t \n";
        assert_eq!(
            compliance_options(input),
            Ok((
                "",
                ComplianceOptions {
                    regex: true,
                    state: StateOption::Optional,
                    r#match: MatchOption::First
                }
            ))
        );

        let input = "#[regex]\n#[state=invalid]\n#[match=all]\n";
        assert!(compliance_options(input).is_err());
    }
}
