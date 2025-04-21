use std::str::FromStr;

use crate::{
    compliance::options::{ComplianceOptions, ComplianceOptionsContainer},
    error::FlatConfigError,
    parse::{misc::ParseOption, parse_configuration, ItemsContainer},
};

#[derive(Debug, Default, Clone)]
pub struct FlatConfig {
    // raw_content ?
    // parse_option
    // compliance_option
    pub items: Vec<FlatConfigItem>,
}

impl FlatConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromStr for FlatConfig {
    type Err = FlatConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let options = ParseOption {
            ignore_options: true,
            ..Default::default()
        };
        Ok(parse_configuration(s, Some(options))?)
    }
}

impl ItemsContainer for FlatConfig {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub enum FlatConfigItem {
    Line(FlatConfigLine),
    Parent(FlatConfigParent),
}

impl PartialEq for FlatConfigItem {
    fn eq(&self, other: &Self) -> bool {
        self.get_item_key() == other.get_item_key()
    }
}

impl ComplianceOptionsContainer for FlatConfigItem {
    fn get_options(&self) -> ComplianceOptions {
        match self {
            FlatConfigItem::Line(line) => line.get_options(),
            FlatConfigItem::Parent(parent) => parent.get_options(),
        }
    }

    fn set_options(&mut self, options: ComplianceOptions) {
        match self {
            FlatConfigItem::Line(ref mut line) => {
                line.set_options(options);
            }
            FlatConfigItem::Parent(ref mut parent) => {
                parent.set_options(options);
            }
        }
    }

    #[cfg(debug_assertions)]
    fn get_raw_options(&self) -> &Vec<String> {
        match self {
            FlatConfigItem::Line(line) => line.get_raw_options(),
            FlatConfigItem::Parent(parent) => parent.get_raw_options(),
        }
    }

    #[cfg(debug_assertions)]
    fn set_raw_options(&mut self, options: &[String]) {
        match self {
            FlatConfigItem::Line(ref mut line) => {
                line.set_raw_options(options);
            }
            FlatConfigItem::Parent(ref mut parent) => {
                parent.set_raw_options(options);
            }
        }
    }
}

impl FlatConfigItem {
    pub(crate) fn get_item_key(&self) -> &str {
        match &self {
            FlatConfigItem::Line(line) => &line.line,
            FlatConfigItem::Parent(parent) => &parent.key,
        }
    }

    pub(crate) fn is_variant_eq(&self, other: &Self) -> bool {
        (matches!(self, Self::Parent(_)) && matches!(other, Self::Parent(_))
            || matches!(self, Self::Line(_)) && matches!(other, Self::Line(_)))
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatConfigLine {
    pub index: usize,
    pub line: String,
    #[cfg(debug_assertions)]
    pub raw_options: Vec<String>,
    pub options: ComplianceOptions,
}

impl ComplianceOptionsContainer for FlatConfigLine {
    fn get_options(&self) -> ComplianceOptions {
        self.options
    }

    fn set_options(&mut self, options: ComplianceOptions) {
        self.options = options;
    }

    #[cfg(debug_assertions)]
    fn get_raw_options(&self) -> &Vec<String> {
        &self.raw_options
    }

    #[cfg(debug_assertions)]
    fn set_raw_options(&mut self, options: &[String]) {
        self.raw_options = options.to_vec();
    }
}

impl FlatConfigLine {
    pub fn new(index: usize, line: &str) -> Self {
        Self {
            index,
            line: String::from(line),
            ..Default::default()
        }
    }

    fn from_parent(p: FlatConfigParent) -> Self {
        Self {
            index: p.index,
            line: p.key,
            #[cfg(debug_assertions)]
            raw_options: p.raw_options,
            options: p.options,
        }
    }
}

impl From<FlatConfigParent> for FlatConfigLine {
    fn from(value: FlatConfigParent) -> Self {
        Self::from_parent(value)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatConfigParent {
    pub index: usize,
    pub indent: usize,
    pub key: String,
    pub items: Vec<FlatConfigItem>,
    #[cfg(debug_assertions)]
    raw_options: Vec<String>,
    pub options: ComplianceOptions,
}

impl FlatConfigParent {
    pub fn new(index: usize, indent: usize, key: String) -> Self {
        Self {
            index,
            indent,
            key,
            ..Default::default()
        }
    }

    pub fn new_with_items(
        index: usize,
        indent: usize,
        key: String,
        items: Vec<FlatConfigItem>,
    ) -> Self {
        Self {
            index,
            indent,
            key,
            items,
            ..Default::default()
        }
    }
}

impl ItemsContainer for FlatConfigParent {
    fn get_indent(&self) -> usize {
        self.indent
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

impl ComplianceOptionsContainer for FlatConfigParent {
    fn get_options(&self) -> ComplianceOptions {
        self.options
    }

    fn set_options(&mut self, options: ComplianceOptions) {
        self.options = options;
    }

    #[cfg(debug_assertions)]
    fn get_raw_options(&self) -> &Vec<String> {
        &self.raw_options
    }

    #[cfg(debug_assertions)]
    fn set_raw_options(&mut self, options: &[String]) {
        self.raw_options = options.to_vec();
    }
}
