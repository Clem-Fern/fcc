use std::error::Error;

use clap::Subcommand;
use log::trace;

use crate::Cli;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Retreive configuration from target device(s)
    Get {},

    /// Check configuration compliance for a given policy
    Check {},

    /// Test device connectivity
    Test {},
}

impl ConfigCommands {
    pub fn matches(cli: &Cli, command: &Self) -> Result<(), Box<dyn Error>> {
        match command {
            ConfigCommands::Get {} => config_subcommand_get(cli)?,
            ConfigCommands::Test {} => todo!(),
            ConfigCommands::Check {} => config_subcommand_check(cli)?,
        }
        Ok(())
    }
}

fn config_subcommand_get(_cli: &Cli) -> Result<(), Box<dyn Error>> {
    trace!("config_subcommand_get");

    Ok(())
}

fn config_subcommand_check(_cli: &Cli) -> Result<(), Box<dyn Error>> {
    trace!("config_subcommand_check");

    Ok(())
}
