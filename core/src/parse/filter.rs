use regex::Regex;

use crate::compliance::options::parse::COMPLIANCE_OPTION_REGEX;

use super::misc::ParseOption;

#[derive(Debug, Default, Clone)]
pub(crate) struct FilterOption {
    pub(crate) ignore_options: bool,
    pub(crate) regex_filter: Option<Regex>,
}

impl From<ParseOption> for FilterOption {
    fn from(value: ParseOption) -> Self {
        Self {
            ignore_options: value.ignore_options,
            regex_filter: value.regex_filter,
        }
    }
}

pub(crate) fn filter_line(line: &str, options: Option<FilterOption>) -> bool {
    let options = options.unwrap_or_default();
    if line.trim().is_empty() {
        return false;
    }

    if COMPLIANCE_OPTION_REGEX.is_match(line) {
        return !options.ignore_options;
    }

    if let Some(reg) = options.regex_filter {
        return !reg.is_match(line);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_line_empty() {
        let lines = include_str!("../../test/filter_line/1.txt").lines();
        let filtered_line = lines.filter(|l| filter_line(l, None));

        assert_eq!(filtered_line.count(), 4);
    }

    #[test]
    fn test_filter_line_regex() {
        let custom_filter = Regex::new(r"^\s*!.*$").unwrap();
        let vec = vec![
            "line 1 indent 0",
            " line 2 indent 1",
            "  line 3 indent 2",
            "   line 4 indent 3",
            "    line 5 indent 4",
        ];

        let filter_option = FilterOption {
            regex_filter: Some(custom_filter.clone()),
            ..Default::default()
        };

        let lines = include_str!("../../test/filter_line/2.txt").lines();
        let filtered_line = lines.filter(|l| filter_line(l, Some(filter_option.clone())));

        assert_eq!(filtered_line.collect::<Vec<&str>>(), vec);
    }

    #[test]
    fn test_filter_line_keep_fcc_option() {
        let vec = vec![
            "#[option1]",
            "line 1 indent 1",
            "  #[option2]   ",
            "  line 2 indent 2",
            "  #[option3]",
            " line 3 indent 1",
        ];

        let vec_without_options = vec!["line 1 indent 1", "  line 2 indent 2", " line 3 indent 1"];

        let mut filter_option = FilterOption {
            regex_filter: Some(COMPLIANCE_OPTION_REGEX.clone()),
            ..Default::default()
        };

        let lines = include_str!("../../test/filter_line/3.txt").lines();
        let filtered_line = lines.filter(|l| filter_line(l, Some(filter_option.clone())));

        assert_eq!(filtered_line.collect::<Vec<&str>>(), vec);

        filter_option.ignore_options = true;
        let lines = include_str!("../../test/filter_line/3.txt").lines();
        let filtered_line = lines.filter(|l| filter_line(l, Some(filter_option.clone())));

        assert_eq!(filtered_line.collect::<Vec<&str>>(), vec_without_options);
    }
}
