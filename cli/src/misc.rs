use std::io::IsTerminal;

use log::LevelFilter;

use crate::Cli;

pub fn get_log_level(cli: &Cli) -> LevelFilter {
    if !std::io::stdout().is_terminal() {
        // Output to other program
        return LevelFilter::Off;
    }

    // todo: json output ?

    cli.verbose.log_level_filter()
}