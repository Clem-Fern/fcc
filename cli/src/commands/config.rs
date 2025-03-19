use std::{
    fs::{read_to_string, File},
    io::{stdin, BufReader, IsTerminal, Read},
    path::PathBuf,
    process::ExitCode,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use clap::Subcommand;
use fcc::{compliance::check_compliance, FlatConfig, FlatConfigCompliance};
use log::{debug, error, info};

use crate::Cli;

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Check configuration compliance against one or several policies
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
    pub fn matches(cli: &Cli, command: &Self) -> Result<ExitCode> {
        match command {
            ConfigCommands::Check {
                config,
                policies,
                ignore_invalid_policy,
            } => config_subcommand_check(cli, config, policies, *ignore_invalid_policy),
        }
    }
}

fn config_subcommand_check(
    _cli: &Cli,
    config_path: &PathBuf,
    policies: &[PathBuf],
    ignore_invalid_policy: bool,
) -> Result<ExitCode> {
    debug!("config_subcommand_check config {}", config_path.display());

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

    let config = FlatConfig::from_str(&raw_config)?;

    let mut return_error = false;
    for path in policies {
        debug!(
            "config_subcommand_check policy {} against {}",
            path.display(),
            config_path.display()
        );
        match read_to_string(path) {
            Ok(raw_policy) => match FlatConfigCompliance::from_str(&raw_policy) {
                Ok(fcc) => {
                    let result = check_compliance(fcc, config.clone());
                    for r in result {
                        if r.result.is_err() {
                            return_error = true;
                            error!("{}", r);
                        } else {
                            info!("{}", r);
                        }
                    }
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

    if return_error {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
