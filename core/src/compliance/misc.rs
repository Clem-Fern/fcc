use std::{error, fmt};

use crate::config::FlatConfigItem;

use super::options::ComplianceOptionsContainer;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ItemComplianceResult {
    pub policy: FlatConfigItem,
    pub result: Result<ComplianceOk, ComplianceError>,
}

impl ItemComplianceResult {
    pub fn new_present_ok(policy: FlatConfigItem, item: FlatConfigItem) -> Self {
        Self {
            policy,
            result: Ok(ComplianceOk::IsPresent(item)),
        }
    }

    pub fn new_absent_ok(policy: FlatConfigItem) -> Self {
        Self {
            policy,
            result: Ok(ComplianceOk::IsAbsent),
        }
    }

    pub fn new_present_nok_ok(policy: FlatConfigItem) -> Self {
        Self {
            policy,
            result: Ok(ComplianceOk::OptionalIsAbsent),
        }
    }

    pub fn new_absent_nok(policy: FlatConfigItem, item: FlatConfigItem) -> Self {
        Self {
            policy,
            result: Err(ComplianceError::ShouldBeAbsentIsPresent(item)),
        }
    }

    pub fn new_present_nok(policy: FlatConfigItem) -> Self {
        Self {
            policy,
            result: Err(ComplianceError::ShouldBePresentIsAbsent),
        }
    }
}

impl fmt::Display for ItemComplianceResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let options = self.policy.get_options();
        write!(f, "Policy(match: {}", options.match_type)?;
        if options.regex {
            write!(f, ", regex=true")?;
        }
        write!(f, ") \"{}\" ", self.policy.get_item_key())?;

        match &self.result {
            Ok(result) => match result {
                ComplianceOk::IsPresent(ref item) => {
                    write!(f, "found: \"{}\"", item.get_item_key())?;
                }
                ComplianceOk::OptionalIsAbsent => {
                    write!(f, "no match found but it's ok.")?;
                }
                ComplianceOk::IsAbsent => {
                    write!(f, "nothing found, as it should be.")?;
                }
            },
            Err(err) => match err {
                ComplianceError::ShouldBePresentIsAbsent => {
                    write!(f, "no match found.")?;
                }
                ComplianceError::ShouldBeAbsentIsPresent(ref item) => {
                    write!(
                        f,
                        "found something that should not be there: \"{}\"",
                        item.get_item_key()
                    )?;
                }
            },
        };
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub enum ComplianceOk {
    IsPresent(FlatConfigItem),
    OptionalIsAbsent,
    IsAbsent,
}

impl fmt::Display for ComplianceOk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub enum ComplianceError {
    ShouldBePresentIsAbsent,
    ShouldBeAbsentIsPresent(FlatConfigItem),
}

impl error::Error for ComplianceError {}

impl fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
