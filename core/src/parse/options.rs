use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        char,
        complete::{alpha1, multispace0},
    },
    combinator::{all_consuming, map, map_res},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use strum::Display;

use crate::compliance::options::{MatchOption, StateOption};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum ComplianceOption {
    Regex,
    State(StateOption),
    Match(MatchOption),
}

pub fn parse_compliance_option(input: &str) -> IResult<&str, ComplianceOption> {
    all_consuming(terminated(
        delimited(
            tag("#["),
            alt((
                map(tag("regex"), |_| ComplianceOption::Regex),
                map(
                    preceded(tag("state="), parse_state_option),
                    ComplianceOption::State,
                ),
                map(
                    preceded(tag("match="), parse_match_option),
                    ComplianceOption::Match,
                ),
            )),
            char(']'),
        ),
        multispace0,
    ))
    .parse(input)
}

fn parse_state_option(input: &str) -> IResult<&str, StateOption> {
    map_res(alpha1, |s: &str| StateOption::from_str(s)).parse(input)
}

fn parse_match_option(input: &str) -> IResult<&str, MatchOption> {
    map_res(alpha1, |s: &str| MatchOption::from_str(s)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_match_option() {
        assert_eq!(parse_match_option("all"), Ok(("", MatchOption::All)));
        assert_eq!(parse_match_option("first"), Ok(("", MatchOption::First)));
        assert!(parse_match_option("invalid").is_err());
    }

    #[test]
    fn test_parse_state_option() {
        assert_eq!(
            parse_state_option("present"),
            Ok(("", StateOption::Present))
        );
        assert_eq!(
            parse_state_option("optional"),
            Ok(("", StateOption::Optional))
        );
        assert_eq!(parse_state_option("absent"), Ok(("", StateOption::Absent)));
        assert!(parse_state_option("invalid").is_err());
    }

    #[test]
    fn test_parse_regex_compliance_option() {
        assert_eq!(
            parse_compliance_option("#[regex]"),
            Ok(("", ComplianceOption::Regex))
        );
        assert!(parse_compliance_option("#[regex=]").is_err());
    }

    #[test]
    fn test_parse_match_compliance_option() {
        assert_eq!(
            parse_compliance_option("#[match=first]"),
            Ok(("", ComplianceOption::Match(MatchOption::First)))
        );
        assert_eq!(
            parse_compliance_option("#[match=all]"),
            Ok(("", ComplianceOption::Match(MatchOption::All)))
        );
        assert!(parse_compliance_option("#[match=invalid]").is_err());
    }

    #[test]
    fn test_parse_state_compliance_option() {
        assert_eq!(
            parse_compliance_option("#[state=present]"),
            Ok(("", ComplianceOption::State(StateOption::Present)))
        );
        assert_eq!(
            parse_compliance_option("#[state=optional]"),
            Ok(("", ComplianceOption::State(StateOption::Optional)))
        );
        assert_eq!(
            parse_compliance_option("#[state=absent]"),
            Ok(("", ComplianceOption::State(StateOption::Absent)))
        );
        assert!(parse_compliance_option("#[state=invalid]").is_err());
    }

    #[test]
    fn test_parse_compliance_option_multispace0() {
        assert_eq!(
            parse_compliance_option("#[regex]   "),
            Ok(("", ComplianceOption::Regex))
        );
        assert_eq!(
            parse_compliance_option("#[state=optional] "),
            Ok(("", ComplianceOption::State(StateOption::Optional)))
        );
        assert_eq!(
            parse_compliance_option("#[match=all]                  "),
            Ok(("", ComplianceOption::Match(MatchOption::All)))
        );
        assert!(parse_compliance_option("   #[regex]").is_err());
        assert!(parse_compliance_option("   #[state=optional]").is_err());
        assert!(parse_compliance_option("   #[match=all]").is_err());
    }
}
