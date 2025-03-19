mod commands;
mod misc;

use std::process::ExitCode;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use commands::Commands;
use human_panic::{setup_panic, Metadata};
use log::error;
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

#[tokio::main]
async fn main() -> ExitCode {
    setup_panic!(Metadata::new(
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION")
    ));

    let cli = Cli::parse();

    let _ = stderrlog::new().verbosity(get_log_level(&cli)).init();

    let res = Commands::matches(&cli);
    match res {
        Ok(res) => return res,
        Err(err) => {
            error!("Error: {}", err);
            error!("Try \"fcc -h\"");
            return ExitCode::FAILURE;
        }
    }
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
