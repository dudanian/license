//! Application prelude.
//! 
//! Reexports most of the commonly used types for the subcommands.

pub use clap::{
    App,
    AppSettings,
    Arg,
    ArgMatches,
    SubCommand,
};

pub use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
};

/// `Result` wrapper using `Box<dyn std::errorr::Error>` as `Error`.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// `Result` wrapper using `()` as `Ok`.
pub type Failure = Result<()>;
