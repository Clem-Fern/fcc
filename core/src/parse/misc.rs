use regex::Regex;

pub fn nb_whitespace_at_start(line: &str) -> usize {
    line.chars().take_while(|f| f.is_ascii_whitespace()).count()
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ParseOption {
    pub ignore_options: bool,
    pub regex_filter: Option<Regex>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_whitespace_at_start() {
        assert_eq!(nb_whitespace_at_start("line line line"), 0);
        assert_eq!(nb_whitespace_at_start(" line line line"), 1); // space
        assert_eq!(nb_whitespace_at_start("	line line line"), 1); // tab
        assert_eq!(nb_whitespace_at_start("  line line line"), 2); // space
        assert_eq!(nb_whitespace_at_start("		line line line"), 2); // tab
    }
}
