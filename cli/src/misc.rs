use std::io::IsTerminal;

use log::LevelFilter;

use crate::{commands::Commands, Cli};

pub fn get_log_level(cli: &Cli) -> LevelFilter {
    if !std::io::stdout().is_terminal() {
        // Output to other program
        return LevelFilter::Off;
    }

    // disable log for completion command
    if matches!(cli.command, Commands::Completion { shell: _ }) {
        return LevelFilter::Off;
    }

    // todo: json output ?

    cli.verbose.log_level_filter()
}
