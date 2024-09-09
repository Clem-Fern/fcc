use std::{
    fs::File,
    io::{stdin, BufReader, IsTerminal, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use clap::Subcommand;
use fcc::FlatConfigCompliance;
use log::{info, trace, warn};

use crate::Cli;

#[derive(Subcommand)]
pub enum PolicyCommands {
    /// Check policy file(s) syntax
    Lint {
        /// The path to the policy file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "POLICY", required = true)]
        policies: Vec<PathBuf>,
    },
}

impl PolicyCommands {
    pub fn matches(cli: &Cli, command: &Self) -> Result<()> {
        match command {
            PolicyCommands::Lint { policies } => policy_subcommand_lint(cli, policies)?,
        }
        Ok(())
    }
}

fn policy_subcommand_lint(_cli: &Cli, policies_path: &[PathBuf]) -> Result<()> {
    for path in policies_path {
        trace!("subcommand_lint path {}", path.display());
        let mut data = String::new();

        if path.is_dir() {
            continue;
        }

        if *path == PathBuf::from("-") {
            if stdin().is_terminal() || policies_path.len() != 1 {
                return Err(anyhow!("Reading from stdin one time is enough."));
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
