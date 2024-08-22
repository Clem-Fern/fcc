use std::{
    error::Error,
    fs::File,
    io::{self, stdin, BufReader, IsTerminal, Read},
    path::PathBuf,
};

mod config;

use clap::{CommandFactory, Subcommand};
use clap_complete::{generate, Shell};
use config::ConfigCommands;
use fcc::FlatConfigCompliance;
use log::{info, trace, warn};

use crate::Cli;

#[derive(Subcommand)]
pub enum Commands {
    /// Check policy file validity
    Lint {
        /// The path to the policy file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "POLICY", required = true)]
        policies: Vec<PathBuf>,
    },

    /// Configuration related commands
    Config{
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
    pub fn matches(cli: &Cli) -> Result<(), Box<dyn Error>> {
        match &cli.command {
            Commands::Lint { policies } => subcommand_lint(cli, policies)?,
            Commands::Config { command } => subcommand_config(cli, command)?,
            Commands::Completion { shell } => subcommand_completion(cli, shell)?,
        }
        Ok(())
    }
}

fn subcommand_lint(_cli: &Cli, policies_path: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    for path in policies_path {
        trace!("subcommand_lint path {}", path.display());
        let mut data = String::new();

        if path.is_dir() {
            continue;
        }

        if *path == PathBuf::from("-") {
            if stdin().is_terminal() || policies_path.len() != 1 {
                Cli::command().print_help().unwrap();
                std::process::exit(2);
            }

            let mut read = BufReader::new(stdin().lock());
            read.read_to_string(&mut data)?;
        } else {
            let mut read = BufReader::new(File::open(path)?);
            read.read_to_string(&mut data)?;
        }

        match FlatConfigCompliance::new_from_raw(&data) {
            Ok(_) => {
                info!("{}: Syntax OK.", path.display());
            }
            Err(err) => {
                warn!("{}: {}", path.display(), err);
            }
        }
    }
    Ok(())
}

fn subcommand_config(cli: &Cli, command: &ConfigCommands) -> Result<(), Box<dyn Error>> {
    trace!("subcommand_config");
    ConfigCommands::matches(cli, command)?;
    Ok(())
}

fn subcommand_completion(_cli: &Cli, shell: &Shell) -> Result<(), Box<dyn Error>> {
    generate(
        shell.to_owned(),
        &mut Cli::command(),
        env!("CARGO_BIN_NAME"),
        &mut io::stdout(),
    );
    Ok(())
}
