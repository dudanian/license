//! # apply sumcommand
//!
//! Applies the provided license to the current directory, resulting
//! in the creation of a `LICENSE.txt` file.
//!
//! ```bash
//! $ license apply MIT
//! $ ls
//! LICENSE.txt
//! ```

use crate::prelude::*;

use std::env;
use std::fs::File;
use std::io;

use license::net;
use license::License;

/// Build the subcommand.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("apply")
        .about("Initialize (testing)")
        .settings(&[AppSettings::UnifiedHelpMessage])
        .arg(
            Arg::with_name("LICENSE")
                .help("Name of license to apply")
                .required(true),
        )
}

/// Run the subcommand.
pub fn exec(args: &ArgMatches) -> Failure {
    let license = args.value_of("LICENSE").unwrap();

    let license = License::from(license)?;
    let path = license.path();

    if !path.exists() {
        println!("Downloading from: {:?}", license.url());
        net::download(license.url(), path)?;
    }

    let mut dst = env::current_dir()?;

    let filename = "LICENSE.txt";
    dst.push(filename);
    if dst.exists() {
        return Err(format!("file {:?} already exists!", dst).into());
    }

    io::copy(&mut File::open(path)?, &mut File::create(dst)?)?;
    Ok(())
}
