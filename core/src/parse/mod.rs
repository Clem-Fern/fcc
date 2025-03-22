pub mod error;
pub mod filter;
pub(crate) mod misc;
use std::{cmp::Ordering, iter::Peekable};

use error::ParseError;
use filter::filter_line;
use log::trace;
use misc::{nb_whitespace_at_start, ParseOption};

use crate::{
    compliance::options::parse::process_fcc_options,
    config::{FlatConfigItem, FlatConfigLine, FlatConfigParent},
};

pub trait ItemsContainer {
    fn get_indent(&self) -> usize;
    fn get_items(&self) -> &Vec<FlatConfigItem>;

    fn set_items(&mut self, items: &[FlatConfigItem]);
    fn push_item(&mut self, item: &FlatConfigItem);
    fn appends_items(&mut self, items: &[FlatConfigItem]);
    fn pop_last_item(&mut self) -> Option<FlatConfigItem>;
}

pub(crate) fn parse_configuration<F>(
    raw_config: &str,
    options: Option<ParseOption>,
) -> Result<F, ParseError>
where
    F: ItemsContainer + Default,
{
    let options = options.unwrap_or_default();
    let mut parent = F::default();
    let lines = raw_config
        .lines()
        .map(String::from)
        .filter(|l| filter_line(l, Some(options.clone().into())))
        .enumerate();

    let mut lines = lines
        .filter(|(_, l)| {
            if l.trim().is_empty() {
                return false;
            }
            true
        })
        .peekable();

    process_next_indent_level(&mut lines, &mut parent)?;

    if !options.ignore_options {
        process_fcc_options(&mut parent)?;
    }

    Ok(parent)
}

pub(crate) fn process_next_indent_level(
    vals: &mut Peekable<impl Iterator<Item = (usize, String)> + Clone>,
    previous_parent: &mut dyn ItemsContainer,
) -> Result<(), ParseError> {
    let previous_parent_indent = previous_parent.get_indent();
    let nb_same_indent = vals
        .clone()
        .take_while(|(_, t)| nb_whitespace_at_start(t) == previous_parent_indent)
        .count();

    let same_indent: Vec<FlatConfigItem> = vals
        .take(nb_same_indent)
        .map(|(i, f) | FlatConfigItem::Line(FlatConfigLine::new(i, &f[previous_parent_indent..f.len()])))
        .collect();

    trace!(
        "Take While (indent {}): found {}",
        previous_parent.get_indent(),
        same_indent.len()
    );

    previous_parent.appends_items(&same_indent);

    while let Some((i, next_val)) = vals.peek() {
        let indent = nb_whitespace_at_start(next_val);

        trace!(
            "next value: indent {}, found: {}, compare with previous indent {}",
            indent,
            next_val,
            previous_parent.get_indent()
        );

        match indent.cmp(&previous_parent_indent) {
            Ordering::Equal => {
                trace!("next value: indent {}, found: {}, compare with previous indent {}: continue process with the same indent", indent, next_val, previous_parent.get_indent());
                process_next_indent_level(vals, previous_parent)?;
            }
            Ordering::Greater => {
                if previous_parent.get_items().is_empty() {
                    // As this is call when indent change, previous parent can't be empty
                    return Err(ParseError::BadIndentation(String::from(next_val)));
                }
                trace!("next value: indent {}, found: {}, compare with previous indent {}: get or create parent", indent, next_val, previous_parent.get_indent());
                let previous_item = previous_parent.pop_last_item().unwrap();
                let mut parent = match previous_item {
                    FlatConfigItem::Line(line) => FlatConfigParent::new(*i, indent, line.line),
                    FlatConfigItem::Parent(_) => {
                        trace!("next value: indent {}, found: {}, compare with previous indent {}: already a parent", indent, next_val, previous_parent.get_indent());
                        return Err(ParseError::BadIndentation(String::from(next_val)));
                    }
                };
                process_next_indent_level(vals, &mut parent)?;
                previous_parent.push_item(&FlatConfigItem::Parent(parent));
            }
            Ordering::Less => {
                trace!("next value: indent {}, found: {}, compare with previous indent {}: get out of process", indent, next_val, previous_parent.get_indent());
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        compliance::options::{ComplianceOptions, ComplianceOptionsContainer},
        config::FlatConfig,
    };

    use super::*;

    #[test]
    fn test_process_next_indent_level_empty() {
        let mut config = FlatConfigParent::new(0, 1, String::from("test"));
        let mut lines = include_str!("../../test/process_next_indent_level/1.txt")
            .lines()
            .map(String::from)
            .enumerate()
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        assert!(config.items.is_empty());
    }

    #[test]
    fn test_process_next_indent_level_1() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../test/process_next_indent_level/1.txt")
            .lines()
            .map(String::from)
            .enumerate()
            .peekable();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        assert_eq!(config.indent, 0);
        assert_eq!(config.key, String::default());
        assert_eq!(config.items.len(), 3);

        let sub_config = config.items.get(1).unwrap();
        assert!(matches!(sub_config, FlatConfigItem::Parent(_)));
        if let FlatConfigItem::Parent(parent) = sub_config {
            assert_eq!(parent.indent, 1);
            assert_eq!(parent.key, String::from("line 2 indent 0"));
            assert_eq!(parent.items.len(), 1);
        }
    }

    #[test]
    fn test_process_next_indent_level_2() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../test/process_next_indent_level/2.txt")
            .lines()
            .map(String::from)
            .enumerate()
            .peekable();
        let lines_count = lines.clone().count();

        process_next_indent_level(&mut lines, &mut config).unwrap();

        let mut item = &config;
        for n in 0..lines_count - 2 {
            let parent = item.items.first().unwrap();
            assert!(matches!(parent, FlatConfigItem::Parent(_)));
            if let FlatConfigItem::Parent(parent) = parent {
                item = parent;
                assert_eq!(parent.indent, n + 1);
                assert_eq!(parent.items.len(), 1);
            }
        }
    }

    #[test]
    fn test_process_next_indent_level_3() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../test/process_next_indent_level/3.txt")
            .lines()
            .map(String::from)
            .enumerate()
            .peekable();

        let err = process_next_indent_level(&mut lines, &mut config).unwrap_err();
        assert!(matches!(err, ParseError::BadIndentation(_)));
    }

    #[test]
    fn test_process_next_indent_level_4() {
        let mut config = FlatConfigParent::default();
        let mut lines = include_str!("../../test/process_next_indent_level/4.txt")
            .lines()
            .map(String::from)
            .enumerate()
            .peekable();

        let err = process_next_indent_level(&mut lines, &mut config).unwrap_err();
        assert!(matches!(err, ParseError::BadIndentation(_)));
    }

    #[test]
    fn test_parse_configuration() {
        let raw = concat!("#[regex]\n", "line 1\n", " line 2\n", "line 3",);

        let config: FlatConfig = parse_configuration(raw, None).unwrap();

        assert_eq!(config.items.len(), 2);
        let item = config.items.first().unwrap();
        assert!(item.get_options().regex);
        assert!(matches!(item, FlatConfigItem::Parent(_)));
        if let FlatConfigItem::Parent(parent) = item {
            assert_eq!(parent.items.len(), 1);
        }
    }

    #[test]
    fn test_parse_configuration_ignore_options() {
        let options = ParseOption {
            ignore_options: true,
            ..Default::default()
        };

        let raw = concat!("#[regex]\n", "line 1\n", " line 2\n", "line 3",);

        let config: FlatConfig = parse_configuration(raw, Some(options)).unwrap();

        assert_eq!(config.items.len(), 2);
        let item = config.items.first().unwrap();
        assert_eq!(item.get_options(), ComplianceOptions::default());
    }
}
