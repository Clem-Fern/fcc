use std::{io, str::FromStr};

use options::{ComplianceOptionsContainer, MatchOption};
use regex::Regex;

use crate::{
    config::{FlatConfig, FlatConfigItem},
    error::FlatConfigError,
    parse::{parse_configuration, ItemsContainer},
};

pub(crate) mod misc;
pub(crate) mod options;
pub use misc::ItemComplianceResult;

//TODO: compliance result
//TODO: ref result display/format
pub fn check_compliance(
    policy: FlatConfigCompliance,
    config: FlatConfig,
) -> Vec<ItemComplianceResult> {
    process_parent_compliance_check(&policy, &config)
}

fn process_parent_compliance_check(
    policies: &(impl ItemsContainer + ComplianceOptionsContainer),
    same_level_item: &impl ItemsContainer,
) -> Vec<ItemComplianceResult> {
    let mut compliance_result: Vec<ItemComplianceResult> = vec![];
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

                if let Some(regex) = regex.clone() {
                    eq = regex.is_match(f.get_item_key());
                }

                if matches!(item_options.match_type, MatchOption::Present) {
                    // is_variant_eq check eq enum variant type
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
) -> Vec<ItemComplianceResult> {
    let mut compliance_result: Vec<ItemComplianceResult> = vec![];

    let match_type = item.get_options().match_type;
    match match_type {
        MatchOption::Present | MatchOption::Optional => {
            if matches.is_empty() {
                if matches!(match_type, MatchOption::Optional) {
                    compliance_result.push(ItemComplianceResult::new_present_nok_ok(item.clone()));
                } else {
                    compliance_result.push(ItemComplianceResult::new_present_nok(item.clone()));
                }
            } else {
                for matching_item in matches {
                    compliance_result.push(ItemComplianceResult::new_present_ok(
                        item.clone(),
                        matching_item.clone(),
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
                compliance_result.push(ItemComplianceResult::new_absent_ok(item.clone()));
            } else {
                for matching_item in matches {
                    let matching_item = matching_item.clone();
                    compliance_result.push(ItemComplianceResult::new_absent_nok(
                        item.clone(),
                        matching_item.clone(),
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
}

impl FromStr for FlatConfigCompliance {
    type Err = FlatConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(FlatConfigError::IO(io::Error::new(
                io::ErrorKind::Other,
                "Input policy is empty.",
            )));
        }

        Ok(parse_configuration(s, None)?)
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

// TODO: remove this ?
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
