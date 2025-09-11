use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{char, complete::multispace0},
    combinator::{all_consuming, map},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

use crate::compliance::options::{MatchOption, StateOption};

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
    alt((
        map(tag("present"), |_| StateOption::Present),
        map(tag("optional"), |_| StateOption::Optional),
        map(tag("absent"), |_| StateOption::Absent),
    ))
    .parse(input)
}

fn parse_match_option(input: &str) -> IResult<&str, MatchOption> {
    alt((
        map(tag("present"), |_| MatchOption::All),
        map(tag("optional"), |_| MatchOption::First),
    ))
    .parse(input)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
