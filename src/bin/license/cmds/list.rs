//! # list subcommand
//!
//! List all known license identifiers. These should be SPDX
//! identifiers and is not expected to return all possible valid
//! identifiers, but a collection of commonly used licenses
//! plus any previously downloaded licenses.
//!
//! ```bash
//! $ license list
//!   MIT
//!   Apache-2.0
//!   ...
//! ```

use crate::prelude::*;

use license::dirs;

/// Build the subcommand.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("List all known license identifiers")
        .settings(&[AppSettings::UnifiedHelpMessage])
}

/// Run the subcommand.
pub fn exec(_matches: &ArgMatches) -> Failure {
    let dir = dirs::data_dir()?;
    let files = dirs::filenames(&dir)?;

    // TODO print known licenses in addition to downloaded
    for license in files.iter().map(|e| e.file_stem().unwrap()) {
        // XXX should do better than lossy printing
        println!("  {}", license.to_string_lossy());
    }

    Ok(())
}
