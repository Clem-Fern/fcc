use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use crate::{config::FlatConfigItem, parse::ItemsContainer};

use super::{error::ParseError, ComplianceOptions, ComplianceOptionsContainer, MatchOption};

lazy_static! {
    // USE TO CAPTURE OPTION
    pub static ref COMPLIANCE_OPTION_REGEX: Regex = Regex::new(r"^[^\S\r\n]*#\[(?<option>\w+)(=(?<arg>\w+))?][^\S\r\n]*$").unwrap();
}

pub(crate) fn process_fcc_options(parent: &mut dyn ItemsContainer) -> Result<(), ParseError> {
    let mut items: Vec<FlatConfigItem> = vec![];
    let mut item_options: Vec<String> = vec![];

    for i in parent.get_items() {
        let item = i.to_owned().clone();
        let key = item.get_item_key();

        if COMPLIANCE_OPTION_REGEX.is_match(key) {
            if matches!(item, FlatConfigItem::Parent(_)) {
                // Prevent something like that
                //
                // #[debug]
                //     line1
                //
                return Err(ParseError::BadIndentation(String::from(key)));
            }
            item_options.push(String::from(key));
            continue;
        }
        // item does not match COMPLIANCE_OPTION_REGEX

        let mut item_with_options = item;
        // Parse list of string options into ComplianceOptions
        if !item_options.is_empty() {
            #[cfg(debug_assertions)]
            item_with_options.set_raw_options(&item_options);
            let compliance_options = ComplianceOptions::new_from_vec(&item_options)?;
            item_with_options.set_options(compliance_options);
            item_options.clear();
        }

        // Check regex synthax
        if item_with_options.get_options().regex {
            Regex::new(&format!("^{}$", item_with_options.get_item_key())).map_err(|err| {
                ParseError::InvalidRegex(err, item_with_options.get_item_key().to_string())
            })?;
        }

        if let FlatConfigItem::Parent(ref mut parent) = item_with_options {
            if matches!(parent.get_options().match_type, MatchOption::Absent) {
                // If parent must be absent, ignore children items
                item_with_options = FlatConfigItem::Line(parent.clone().into())
            } else {
                process_fcc_options(parent)?;
                if parent.items.is_empty() {
                    item_with_options = FlatConfigItem::Line(parent.clone().into())
                }
            }
        }

        items.push(item_with_options);
    }

    parent.set_items(&items);

    Ok(())
}

pub(super) fn parse_raw_options(
    compliance_option: &mut ComplianceOptions,
    raw_options: &[String],
) -> Result<(), ParseError> {
    for option in raw_options {
        if let Some(caps) = COMPLIANCE_OPTION_REGEX.captures(option) {
            let o = &caps["option"];
            match o {
                "regex" => {
                    compliance_option.regex = true;
                }
                "match" => {
                    if let Some(arg) = caps.name("arg") {
                        compliance_option.match_type = MatchOption::from_str(arg.as_str())
                            .map_err(|_| {
                                ParseError::InvalidOptionArgument(
                                    String::from(arg.as_str()),
                                    String::from(option),
                                )
                            })?;
                    } else {
                        return Err(ParseError::MalformedOption(String::from(option)));
                    }
                }
                #[cfg(debug_assertions)]
                "debug" => {
                    continue;
                }
                _ => return Err(ParseError::UnknowOption(String::from(o))),
            }
        } else {
            return Err(ParseError::MalformedOption(String::from(option)));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(debug_assertions))]
    use crate::{config::FlatConfigParent, parse::process_next_indent_level};

    #[cfg(debug_assertions)]
    use crate::{
        compliance::options::ComplianceOptionsContainer,
        config::FlatConfigParent,
        parse::{filter::filter_line, process_next_indent_level},
    };

    #[test]
    #[cfg(debug_assertions)]
    fn test_process_fcc_options_empty() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/1.txt")
            .lines()
            .filter(|f| !COMPLIANCE_OPTION_REGEX.is_match(f))
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        process_fcc_options(&mut config).unwrap();

        for item in config.items {
            assert!(item.get_raw_options().is_empty())
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    fn test_process_fcc_options_1() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/1.txt")
            .lines()
            .filter(|l| filter_line(l, None))
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        process_fcc_options(&mut config).unwrap();

        for i in 0..config.items.len() - 1 {
            let item = config.items.get(i).unwrap();
            assert_eq!(item.get_raw_options().len(), i);
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    fn test_process_fcc_options_2() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/2.txt")
            .lines()
            .filter(|l| filter_line(l, None))
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        process_fcc_options(&mut config).unwrap();

        let item1 = config.items.get(0).unwrap();
        assert_eq!(item1.get_raw_options().len(), 1);
        if let FlatConfigItem::Parent(parent) = item1 {
            let item2 = parent.items.get(0).unwrap();
            assert_eq!(item2.get_raw_options().len(), 1);
            if let FlatConfigItem::Parent(parent) = item2 {
                let item3 = parent.items.get(0).unwrap();
                assert_eq!(item3.get_raw_options().len(), 1);
            } else {
                assert!(false)
            }
        } else {
            assert!(false)
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    fn test_process_fcc_options_parent_into_line() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/3.txt")
            .lines()
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        process_fcc_options(&mut config).unwrap();

        assert!(matches!(
            config.items.get(0).unwrap(),
            FlatConfigItem::Line(_)
        ));
    }

    #[test]
    #[cfg(debug_assertions)]
    fn test_process_fcc_options_bad_indentation() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/4.txt")
            .lines()
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        let err = process_fcc_options(&mut config).unwrap_err();

        assert!(matches!(err, ParseError::BadIndentation(_)));
    }

    #[test]
    fn test_process_fcc_options_absent_parent_into_line() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/5.txt")
            .lines()
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();
        process_fcc_options(&mut config).unwrap();

        assert_eq!(config.items.len(), 1);
        assert!(matches!(
            config.items.first().unwrap(),
            FlatConfigItem::Line(_)
        ));
    }

    #[test]
    fn test_process_fcc_options_invalid_regex() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../../test/process_fcc_options/6.txt")
            .lines()
            .map(String::from)
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();
        let result = process_fcc_options(&mut config);

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ParseError::InvalidRegex(_, _)
        ));
    }

    #[test]
    fn test_parse_fcc_options_1() {
        let options = vec![
            String::from(" #[regex]   "),
            String::from("     #[match=present]  "),
        ];

        let options = ComplianceOptions::new_from_vec(&options);
        assert!(options.is_ok());
        let options = options.unwrap();

        assert!(options.regex);
        assert!(matches!(options.match_type, MatchOption::Present));
    }

    #[test]
    fn test_parse_fcc_options_2() {
        let options = vec![
            String::from(" #[regex]   "),
            String::from("     #[match=absent]  "),
        ];

        let options = ComplianceOptions::new_from_vec(&options);
        assert!(options.is_ok());
        let options = options.unwrap();

        assert!(options.regex);
        assert!(matches!(options.match_type, MatchOption::Absent));
    }

    #[test]
    fn test_parse_fcc_options_3() {
        let options = vec![
            String::from(" #[regex]   "),
            String::from("     #[match=optional]  "),
        ];

        let options = ComplianceOptions::new_from_vec(&options);
        assert!(options.is_ok());
        let options = options.unwrap();

        assert!(options.regex);
        assert!(matches!(options.match_type, MatchOption::Optional));
    }

    #[test]
    fn test_parse_fcc_options_malformed() {
        let err = ComplianceOptions::new_from_vec(&vec![String::from(" lkjhlkjh   ")]).unwrap_err();
        assert!(matches!(err, ParseError::MalformedOption(_)));

        let err = ComplianceOptions::new_from_vec(&vec![String::from(" #[match]   ")]).unwrap_err();
        assert!(matches!(err, ParseError::MalformedOption(_)));
    }

    #[test]
    fn test_parse_fcc_options_unknow() {
        let options = vec![String::from(" #[option1]   ")];

        let err = ComplianceOptions::new_from_vec(&options).unwrap_err();
        assert!(matches!(err, ParseError::UnknowOption(_)));
    }

    #[test]
    fn test_parse_fcc_options_invalid_arg() {
        let err =
            ComplianceOptions::new_from_vec(&vec![String::from(" #[match=arg1]   ")]).unwrap_err();
        assert!(matches!(err, ParseError::InvalidOptionArgument(_, _)));
    }
}
