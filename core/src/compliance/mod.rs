use std::io;

use options::{ComplianceOptionsContainer, MatchOption};
use regex::Regex;

use crate::{
    config::{FlatConfig, FlatConfigItem},
    error::FlatConfigError,
    parse::{parse_configuration, ItemsContainer},
};

pub(crate) mod misc;
pub(crate) mod options;
pub use misc::ComplianceResult;

//TODO: compliance result
pub fn check_compliance(
    policy: FlatConfigCompliance,
    config: FlatConfig,
) -> Result<(), FlatConfigError> {
    process_parent_compliance_check(&policy, &config);
    Ok(())
}

fn process_parent_compliance_check(
    policies: &(impl ItemsContainer + ComplianceOptionsContainer),
    same_level_item: &impl ItemsContainer,
) -> Vec<ComplianceResult> {
    let mut compliance_result: Vec<ComplianceResult> = vec![];
    let mut same_level_items = same_level_item.get_items().clone();
    for item in policies.get_items() {
        let item_options = item.get_options();
        let current_same_level_items = same_level_items.clone();

        let regex = if item_options.regex {
            let regex = Regex::new(&format!("^{}$", item.get_item_key())).unwrap();
            Some(regex)
        } else {
            None
        };

        let matching_items: Vec<&FlatConfigItem> = current_same_level_items
            .iter()
            .filter(|f| -> bool {
                let mut eq = item.eq(f);

                if item_options.regex {
                    eq = regex.clone().unwrap().is_match(f.get_item_key());
                }

                if matches!(item_options.match_type, MatchOption::Present) {
                    eq = eq && item.is_variant_eq(f);
                }

                eq
            })
            .collect();

        let mut cr = process_item_matches_compliance(item, matching_items.clone());
        compliance_result.append(&mut cr);

        if !matching_items.is_empty() {
            // items can match only once
            if item_options.regex {
                // remove already matched items
                same_level_items = current_same_level_items
                    .clone()
                    .into_iter()
                    .filter(|f| -> bool { !matching_items.contains(&f) })
                    .collect::<Vec<FlatConfigItem>>();
            } else {
                // if not regex, only remove first matched item
                let first_match = matching_items.first().unwrap(); // safe
                let mut found_match = false;
                same_level_items = same_level_items
                    .into_iter()
                    .filter(|f| -> bool {
                        if !found_match && f.eq(first_match) && f.is_variant_eq(first_match) {
                            found_match = true;
                            return false;
                        }
                        true
                    })
                    .collect::<Vec<FlatConfigItem>>();
            }
        }
    }

    compliance_result
}

fn process_item_matches_compliance(
    item: &FlatConfigItem,
    matches: Vec<&FlatConfigItem>,
) -> Vec<ComplianceResult> {
    let mut compliance_result: Vec<ComplianceResult> = vec![];

    let match_type = item.get_options().match_type;
    match match_type {
        MatchOption::Present => {
            if matches.is_empty() {
                compliance_result.push(ComplianceResult::new_present_nok(item.clone(), None));
            } else {
                for matching_item in matches {
                    compliance_result.push(ComplianceResult::new_ok(
                        item.clone(),
                        Some(matching_item.clone()),
                    ));

                    if let FlatConfigItem::Parent(parent) = item {
                        if let FlatConfigItem::Parent(matching_parent) = matching_item {
                            let mut cr = process_parent_compliance_check(parent, matching_parent);
                            compliance_result.append(&mut cr);
                        }
                    }
                }
            }
        }
        MatchOption::Absent => {
            if matches.is_empty() {
                compliance_result.push(ComplianceResult::new_ok(item.clone(), None));
            } else {
                for matching_item in matches {
                    let matching_item = matching_item.clone();
                    compliance_result.push(ComplianceResult::new_absent_nok(
                        item.clone(),
                        Some(matching_item.clone()),
                    ))
                }
            }
        }
    }
    compliance_result
}

#[derive(Debug, Default, Clone)]
pub struct FlatConfigCompliance {
    // raw_content
    // compliance_option
    pub items: Vec<FlatConfigItem>,
}

impl FlatConfigCompliance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_raw(raw_config: &str) -> Result<Self, FlatConfigError> {
        if raw_config.trim().is_empty() {
            return Err(FlatConfigError::IO(io::Error::new(io::ErrorKind::Other, "Input policy is empty.")));
        }

        Ok(parse_configuration(raw_config, None)?)
    }
}

impl ItemsContainer for FlatConfigCompliance {
    fn get_indent(&self) -> usize {
        0
    }

    fn get_items(&self) -> &Vec<FlatConfigItem> {
        &self.items
    }

    fn push_item(&mut self, item: &FlatConfigItem) {
        self.items.push(item.clone());
    }

    fn appends_items(&mut self, items: &[FlatConfigItem]) {
        self.items.append(&mut items.to_owned());
    }

    fn pop_last_item(&mut self) -> Option<FlatConfigItem> {
        self.items.pop()
    }

    fn set_items(&mut self, items: &[FlatConfigItem]) {
        self.items = items.to_vec();
    }
}

impl ComplianceOptionsContainer for FlatConfigCompliance {
    fn get_options(&self) -> options::ComplianceOptions {
        todo!()
    }

    fn set_options(&mut self, _options: options::ComplianceOptions) {
        todo!()
    }

    #[cfg(debug_assertions)]
    fn get_raw_options(&self) -> &Vec<String> {
        todo!()
    }

    #[cfg(debug_assertions)]
    fn set_raw_options(&mut self, _options: &[String]) {
        todo!()
    }
}

#[cfg(test)]
mod test;