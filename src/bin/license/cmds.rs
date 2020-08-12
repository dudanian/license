//! Subcommands for the application.
//!
//! Each subcommand is implemented in it's own submodule.
//! See the nested submodules for more information on
//! specific commands.

use crate::prelude::*;

mod apply;
mod list;
mod read;

/// Returns currently supported subcommands.
pub fn subcommands<'a, 'b>() -> Vec<App<'a, 'b>> {
    vec![
        apply::subcommand(),
        list::subcommand(),
        read::subcommand(),
    ]
}

/// Calls the subexec for the matched subcommand.
pub fn subexec(subcmd: (&str, Option<&ArgMatches>)) -> Failure {
    match subcmd {
        ("apply", Some(m)) => apply::exec(m),
        ("list", Some(m)) => list::exec(m),
        ("read", Some(m)) => read::exec(m),
        _ => Err("no matching command found".into()),
    }
}
