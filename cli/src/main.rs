use std::{
    error::Error,
    fs::File,
    io::{self, stdin, BufReader, IsTerminal, Read},
    path::PathBuf,
};

mod misc;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use fcc::FlatConfigCompliance;
use human_panic::{setup_panic, Metadata};
use log::{error, info, trace, warn};
use misc::get_log_level;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"))]
#[command(version, about, long_about = None)]
struct Cli {
    // Verbose
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,

    // Subcommand
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check policy file validity
    Lint {
        /// The path to the policy file to read, use - to read from stdin (must not be a tty)
        #[arg(value_name = "POLICY", required = true)]
        policies: Vec<PathBuf>,
    },

    /// shell completion
    ComplianceCheck,

    /// shell completion
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() {
    setup_panic!(Metadata::new(
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION")
    ));

    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(get_log_level(&cli))
        .init();

    let res = match &cli.command {
        Commands::Lint { policies } => {
            subcommand_lint(&cli, policies)
        }
        Commands::ComplianceCheck => todo!(),
        Commands::Completion { shell } => {
            generate(
                shell.to_owned(),
                &mut Cli::command(),
                env!("CARGO_BIN_NAME"),
                &mut io::stdout(),
            );
            Ok(())
        }
    };

    if res.is_err() {
        error!("FCC Error: {}", res.err().unwrap());
        std::process::exit(2);
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
            },
            Err(err) => {
                warn!("{}: {}", path.display(), err);
            },
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
