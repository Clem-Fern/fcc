use std::{
    fs::{read_to_string, File},
    io::{stdin, BufReader, IsTerminal, Read},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use clap::Subcommand;
use fcc::{compliance::check_compliance, FlatConfig, FlatConfigCompliance};
use log::{error, trace};

use crate::Cli;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Check configuration compliance for a given policy
    Check {
        /// The path to the configuration file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "CONFIG", required = true)]
        config: PathBuf,

        /// The path to the policy file to use to check compliance, accept multiple paths
        #[arg(value_name = "POLICY", required = true)]
        policies: Vec<PathBuf>,

        /// Skip error when reading policy file
        #[arg(short, long, action)]
        ignore_invalid_policy: bool,
    },
}

impl ConfigCommands {
    pub fn matches(cli: &Cli, command: &Self) -> Result<()> {
        match command {
            ConfigCommands::Check {
                config,
                policies,
                ignore_invalid_policy,
            } => config_subcommand_check(cli, config, policies, *ignore_invalid_policy)?,
        }
        Ok(())
    }
}

fn config_subcommand_check(
    _cli: &Cli,
    config_path: &PathBuf,
    policies_path: &[PathBuf],
    ignore_invalid_policy: bool,
) -> Result<()> {
    trace!("config_subcommand_check config {}", config_path.display());

    if config_path.is_dir() {
        return Err(anyhow!(
            "CONFIG can't be a directory. {}",
            config_path.display()
        ));
    }

    let mut raw_config = String::new();
    if *config_path == PathBuf::from("-") {
        if stdin().is_terminal() {
            return Err(anyhow!("use - to read from stdin (must not be a tty)."));
        }

        let mut read = BufReader::new(stdin().lock());
        read.read_to_string(&mut raw_config)?;
    } else {
        let mut read = BufReader::new(File::open(config_path)?);
        read.read_to_string(&mut raw_config)?;
    }

    let config = FlatConfig::new_from_raw(&raw_config)?;

    for path in policies_path {
        trace!("config_subcommand_check policy {}", path.display());
        match read_to_string(path) {
            Ok(raw_policy) => match FlatConfigCompliance::new_from_raw(&raw_policy) {
                Ok(fcc) => {
                    let _ = check_compliance(fcc, config.clone());
                }
                Err(err) => {
                    if ignore_invalid_policy {
                        error!(
                            "Unable to parse policy: {}. Use \"lint\" to see details.",
                            path.display()
                        );
                        continue;
                    } else {
                        return Err(anyhow!(err));
                    }
                }
            },
            Err(err) => {
                if ignore_invalid_policy {
                    error!("Unable to read policy: {}", path.display());
                    continue;
                } else {
                    return Err(anyhow!(err));
                }
            }
        }
    }
    Ok(())
}
