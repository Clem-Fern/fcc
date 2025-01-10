use std::{
    fs::{read_to_string, File},
    io::{stdin, BufReader, IsTerminal, Read},
    path::{Path, PathBuf},
    process::ExitCode,
    str::FromStr,
};

use anyhow::{anyhow, Result};
use clap::Subcommand;
use fcc::{compliance::check_compliance, FlatConfig, FlatConfigCompliance};
use log::{debug, error, info, warn};

use crate::Cli;

#[derive(Subcommand)]
pub enum PolicyCommands {
    /// Verify policy file(s) syntax
    Lint {
        /// The path to the policy file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "POLICY", required = true)]
        policies: Vec<PathBuf>,
    },

    /// Apply policy rules on one or several configurations
    Check {
        /// The path to the policy file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "POLICY", required = true)]
        policy: PathBuf,

        /// The path to the configuration file to use, accept multiple paths
        #[arg(value_name = "CONFIG", required = true)]
        configs: Vec<PathBuf>,

        /// Skip error when reading config file
        #[arg(short, long, action)]
        ignore_invalid_config: bool,
    },
}

impl PolicyCommands {
    pub fn matches(cli: &Cli, command: &Self) -> Result<ExitCode> {
        match command {
            PolicyCommands::Lint { policies } => policy_subcommand_lint(cli, policies),
            PolicyCommands::Check {
                policy,
                configs,
                ignore_invalid_config,
            } => policy_subcommand_check(cli, policy, configs, *ignore_invalid_config),
        }
    }
}

fn policy_subcommand_lint(_cli: &Cli, policies: &[PathBuf]) -> Result<ExitCode> {
    for path in policies {
        debug!("policy_subcommand_lint path {}", path.display());
        let mut data = String::new();

        if path.is_dir() {
            continue;
        }

        if *path == PathBuf::from("-") {
            if policies.len() != 1 {
                return Err(anyhow!("Reading from stdin one time is enough."));
            }

            if stdin().is_terminal() {
                return Err(anyhow!("\"-\" nothing to read from there."));
            }

            let mut read = BufReader::new(stdin().lock());
            read.read_to_string(&mut data)?;
        } else {
            let mut read = BufReader::new(File::open(path)?);
            read.read_to_string(&mut data)?;
        }

        match FlatConfigCompliance::from_str(&data) {
            Ok(_) => {
                info!("{}: Syntax OK.", path.display());
            }
            Err(err) => {
                warn!("{}: {}", path.display(), err);
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn policy_subcommand_check(
    _cli: &Cli,
    policy_path: &Path,
    configs: &[PathBuf],
    ignore_invalid_config: bool,
) -> Result<ExitCode> {
    debug!("policy_subcommand_check policy {}", policy_path.display());

    if policy_path.is_dir() {
        return Err(anyhow!(
            "POLICY can't be a directory. {}",
            policy_path.display()
        ));
    }

    let mut raw_policy = String::new();
    if *policy_path == PathBuf::from("-") {
        if stdin().is_terminal() {
            return Err(anyhow!("use - to read from stdin (must not be a tty)."));
        }

        let mut read = BufReader::new(stdin().lock());
        read.read_to_string(&mut raw_policy)?;
    } else {
        let mut read = BufReader::new(File::open(policy_path)?);
        read.read_to_string(&mut raw_policy)?;
    }

    let policy = FlatConfigCompliance::from_str(&raw_policy)?;

    let mut return_error = false;
    for path in configs {
        debug!(
            "policy_subcommand_check policy {} against {}",
            policy_path.display(),
            path.display()
        );
        match read_to_string(path) {
            Ok(raw_config) => match FlatConfig::from_str(&raw_config) {
                Ok(config) => {
                    let result = check_compliance(policy.clone(), config);
                    for r in result {
                        if r.result.is_err() {
                            return_error = true;
                            eprintln!("{}", r);
                            error!("{}", r);
                        } else {
                            println!("{}", r);
                            info!("{}", r);
                        }
                    }
                }
                Err(err) => {
                    if ignore_invalid_config {
                        error!("Unable to parse config: {}.", path.display());
                        continue;
                    } else {
                        return Err(anyhow!(err));
                    }
                }
            },
            Err(err) => {
                if ignore_invalid_config {
                    error!("Unable to read config: {}", path.display());
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
