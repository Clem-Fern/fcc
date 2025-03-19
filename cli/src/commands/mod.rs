use std::{
    io::{self},
    process::ExitCode,
};

mod config;
mod policy;

use anyhow::Result;
use clap::{CommandFactory, Subcommand};
use clap_complete::{generate, Shell};
use config::ConfigCommands;
use log::trace;
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

    /// shell completion
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Commands {
    pub fn matches(cli: &Cli) -> Result<ExitCode> {
        match &cli.command {
            Commands::Policy { command } => subcommand_policy(cli, command),
            Commands::Config { command } => subcommand_config(cli, command),
            Commands::Completion { shell } => subcommand_completion(cli, shell),
        }
    }
}

fn subcommand_policy(cli: &Cli, command: &PolicyCommands) -> Result<ExitCode> {
    trace!("subcommand_policy");
    PolicyCommands::matches(cli, command)
}

fn subcommand_config(cli: &Cli, command: &ConfigCommands) -> Result<ExitCode> {
    trace!("subcommand_config");
    ConfigCommands::matches(cli, command)
}

fn subcommand_completion(_cli: &Cli, shell: &Shell) -> Result<ExitCode> {
    generate(
        shell.to_owned(),
        &mut Cli::command(),
        env!("CARGO_BIN_NAME"),
        &mut io::stdout(),
    );
    Ok(ExitCode::SUCCESS)
}
