use std::{error, fmt};

use crate::config::FlatConfigItem;

#[derive(Debug, Clone)]
pub struct ComplianceResult {
    pub policy: FlatConfigItem,
    pub item: Option<FlatConfigItem>,
    pub result: Result<(), ComplianceError>,
}

impl ComplianceResult {
    pub fn new_ok(policy: FlatConfigItem, item: Option<FlatConfigItem>) -> Self {
        Self {
            policy,
            item,
            result: Ok(()),
        }
    }

    pub fn new_absent_nok(policy: FlatConfigItem, item: Option<FlatConfigItem>) -> Self {
        Self {
            policy,
            item,
            result: Err(ComplianceError::ShouldBeAbsentIsPresent),
        }
    }

    pub fn new_present_nok(policy: FlatConfigItem, item: Option<FlatConfigItem>) -> Self {
        Self {
            policy,
            item,
            result: Err(ComplianceError::ShouldBePresentIsAbsent),
        }
    }
}

impl fmt::Display for ComplianceResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Policy \"{}\" ", self.policy.get_item_key())?;

        if let Some(match_item) = &self.item {
            write!(f, "match \"{}\": ", match_item.get_item_key())?;
        } else {
            write!(f, ": ")?;
        }

        match &self.result {
            Ok(_) => {}
            Err(err) => match err {
                ComplianceError::ShouldBePresentIsAbsent => {
                    write!(f, "no match found.")?;
                }
                ComplianceError::ShouldBeAbsentIsPresent => {
                    write!(f, "it should not be there.")?;
                }
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComplianceError {
    ShouldBePresentIsAbsent,
    ShouldBeAbsentIsPresent,
}

impl error::Error for ComplianceError {}

impl fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
