use std::ops::Not;

use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    combinator::{all_consuming, opt},
    error::{Error, ErrorKind},
    multi::many1,
    sequence::terminated,
    AsChar, IResult, Parser,
};

use crate::{compliance::options::ComplianceOptions, parse::options::compliance_options};

struct ComplianceItem<'a> {
    options: ComplianceOptions,
    content: &'a str,
}

fn compliance_items(input: &str) -> IResult<&str, Vec<ComplianceItem>> {
    all_consuming(many1(compliance_item)).parse(input)
}

fn compliance_item(input: &str) -> IResult<&str, ComplianceItem> {
    terminated((compliance_options, compliance_item_content), opt(newline))
        .map(|(options, content)| ComplianceItem {
            options,
            content: content,
        })
        .parse(input)
}

fn compliance_item_content(input: &str) -> IResult<&str, &str> {
    let (output, content) = take_while(|c: char| c.is_newline().not()).parse(input)?;
    if content.trim().is_empty() {
        return Err(nom::Err::Error(Error::new(input, ErrorKind::Fail)));
    }
    Ok((output, content))
}

#[cfg(test)]
mod tests {
    use crate::compliance::options::{MatchOption, StateOption};

    use super::*;

    #[test]
    fn test_compliance_item_1() {
        let input = "Some content here";
        let result = compliance_item(input);
        assert!(result.is_ok());
        let (remaining, item) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(item.options, ComplianceOptions::default());
        assert_eq!(item.content, "Some content here");
    }

    #[test]
    fn test_compliance_item_2() {
        let input = "#[regex]\nSome content here";
        let result = compliance_item(input);
        assert!(result.is_ok());
        let (remaining, item) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(item.options.regex, true);
        assert_eq!(item.content, "Some content here");
    }

    #[test]
    fn test_compliance_item_3() {
        let input = "#[match=all]\nSome content here\n";
        let result = compliance_item(input);
        assert!(result.is_ok());
        let (remaining, item) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(item.options.regex, false);
        assert_eq!(item.options.r#match, MatchOption::All);
        assert_eq!(item.content, "Some content here");
    }

    #[test]
    fn test_compliance_item_4() {
        let input = "#[state=absent]\nSome content here\nSome other content";
        let result = compliance_item(input);
        assert!(result.is_ok());
        let (remaining, item) = result.unwrap();
        assert_eq!(remaining, "Some other content");
        assert_eq!(item.options.regex, false);
        assert_eq!(item.options.state, StateOption::Absent);
        assert_eq!(item.options.r#match, MatchOption::All);
        assert_eq!(item.content, "Some content here");
    }

    #[test]
    fn test_compliance_item_without_content() {
        let input = "#[regex]\n";
        let result = compliance_item(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_compliance_items_1() {
        let input = "#[regex]\nFirst content\n#[match=all]\nSecond content";
        let result = compliance_items(input);
        let (remaining, items) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].options.regex, true);
        assert_eq!(items[0].content, "First content");
        assert_eq!(items[1].options.r#match, MatchOption::All);
        assert_eq!(items[1].content, "Second content");
    }

    #[test]
    fn test_compliance_items_2() {
        let input =
            "#[regex]\nFirst content\n#[match=all]\nSecond content\nThird content without options";
        let result = compliance_items(input);
        let (remaining, items) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].options.regex, true);
        assert_eq!(items[0].content, "First content");
        assert_eq!(items[1].options.r#match, MatchOption::All);
        assert_eq!(items[1].content, "Second content");
        assert_eq!(items[2].options, ComplianceOptions::default());
        assert_eq!(items[2].content, "Third content without options");
    }

    #[test]
    fn test_compliance_items_3() {
        let input =
            "#[regex]\nFirst content\n#[match=all]\nSecond content\nThird content without options\nFourth content";
        let result = compliance_items(input);
        let (remaining, items) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].options.regex, true);
        assert_eq!(items[0].content, "First content");
        assert_eq!(items[1].options.r#match, MatchOption::All);
        assert_eq!(items[1].content, "Second content");
        assert_eq!(items[2].options, ComplianceOptions::default());
        assert_eq!(items[2].content, "Third content without options");
        assert_eq!(items[3].options, ComplianceOptions::default());
        assert_eq!(items[3].content, "Fourth content");
    }

    #[test]
    fn test_compliance_items_4() {
        let input = "#[regex]\nFirst content\n#[match=all]\n";
        let result = compliance_items(input);
        assert!(result.is_err());
    }
}
