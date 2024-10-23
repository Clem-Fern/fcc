pub mod compliance;
pub mod config;
pub mod error;

#[allow(dead_code)]
pub mod manifest;

pub use compliance::FlatConfigCompliance;
pub use config::FlatConfig;

pub(crate) mod parse;
