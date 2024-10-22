use std::io::{self};

mod config;
mod manifest;
mod policy;

use anyhow::Result;
use clap::{CommandFactory, Subcommand};
use clap_complete::{generate, Shell};
use config::ConfigCommands;
use log::trace;
use manifest::ManifestCommands;
use policy::PolicyCommands;

use crate::Cli;

#[derive(Subcommand)]
pub enum Commands {
    /// Policy related commands    
    Policy {
        #[command(subcommand)]
        command: PolicyCommands,
    },

    /// Configuration related commands
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Manifest related commands
    Manifest {
        #[command(subcommand)]
        command: ManifestCommands,
    },

    /// shell completion
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Commands {
    pub fn matches(cli: &Cli) -> Result<()> {
        match &cli.command {
            Commands::Policy { command } => subcommand_policy(cli, command)?,
            Commands::Config { command } => subcommand_config(cli, command)?,
            Commands::Manifest { command } => subcommand_manifest(cli, command)?,
            Commands::Completion { shell } => subcommand_completion(cli, shell)?,
        }
        Ok(())
    }
}

fn subcommand_policy(cli: &Cli, command: &PolicyCommands) -> Result<()> {
    trace!("subcommand_policy");
    PolicyCommands::matches(cli, command)?;
    Ok(())
}

fn subcommand_config(cli: &Cli, command: &ConfigCommands) -> Result<()> {
    trace!("subcommand_config");
    ConfigCommands::matches(cli, command)?;
    Ok(())
}

fn subcommand_manifest(cli: &Cli, command: &ManifestCommands) -> Result<()> {
    trace!("subcommand_manifest");
    ManifestCommands::matches(cli, command)?;
    Ok(())
}

fn subcommand_completion(_cli: &Cli, shell: &Shell) -> Result<()> {
    generate(
        shell.to_owned(),
        &mut Cli::command(),
        env!("CARGO_BIN_NAME"),
        &mut io::stdout(),
    );
    Ok(())
}
