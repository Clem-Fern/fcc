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
pub enum ManifestCommands {
    /// Verify manifest file(s) syntax
    Lint {
        /// The path to the manifest file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "MANIFEST", required = true)]
        manifests: Vec<PathBuf>,
    },

    /// Check compliance based on manifest options
    Check {
        /// The path to the manifest file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "MANIFEST", required = true)]
        manifests: Vec<PathBuf>,
    },
}

impl ManifestCommands {
    pub fn matches(cli: &Cli, command: &Self) -> Result<()> {
        match command {
            ManifestCommands::Lint { manifests } => manifest_subcommand_lint(cli, manifests)?,
            ManifestCommands::Check { manifests } => manifest_subcommand_check(cli, manifests)?,
        }
        Ok(())
    }
}

fn manifest_subcommand_lint(_cli: &Cli, manifests: &[PathBuf]) -> Result<()> {
    for path in manifests {
        trace!("manifest_subcommand_lint path {}", path.display());
        let mut data = String::new();

        if path.is_dir() {
            continue;
        }

        if *path == PathBuf::from("-") {
            if manifests.len() != 1 {
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

fn manifest_subcommand_check(_cli: &Cli, manifests: &[PathBuf]) -> Result<()> {
    for path in manifests {
        trace!("manifest_subcommand_check path {}", path.display());
        let mut data = String::new();

        if path.is_dir() {
            continue;
        }

        if *path == PathBuf::from("-") {
            if manifests.len() != 1 {
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
