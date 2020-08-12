//! Executable for `license` command to help manage
//! license files.

mod cmds;
mod prelude;

use crate::prelude::*;

/// Builds and executes the application.
pub fn exec() -> Failure {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::UnifiedHelpMessage,
            AppSettings::VersionlessSubcommands,
        ])
        .subcommands(cmds::subcommands())
        .get_matches();

    cmds::subexec(matches.subcommand())
}

/// Prints errors before exiting.
fn main() {
    if let Err(e) = exec() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    };
}
