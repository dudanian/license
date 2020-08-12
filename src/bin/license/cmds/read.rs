//! # read subcommand
//!
//! Prints the license text to standard output.
//! Will attempt to download if no file is found in the local
//! file cache.
//!
//! Might consider using this command in conjunction with a pager:
//!
//! ```bash
//! $ license read MIT
//! MIT License Copyright (c) <year> <copyrigt holders>
//!
//! Permission is hereby granted...
//! ```
//!
//! You might want to use this command in conjuction with a pager:
//!
//! ```bash
//! $ license read MIT | less
//! ```

use crate::prelude::*;

use std::fs::File;
use std::io;

use license::net;
use license::License;

/// Build the subcommand.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("read")
        .about("Print the license text")
        .settings(&[AppSettings::UnifiedHelpMessage])
        .arg(
            Arg::with_name("LICENSE")
                .help("SPDX identifier of license to read")
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

    io::copy(&mut File::open(path)?, &mut io::stdout())?;
    Ok(())
}
