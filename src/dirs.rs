//! Directory and io helper functions.
//! 
//! Mostly wrapping around the `directories` crate.

use directories;
use directories::ProjectDirs;

use std::path::{Path, PathBuf};
use std::io;
use std::fs;

/// Wrapper around `directories::ProjectDirs::from` so we can
/// reuse this with functions below.
fn proj_dirs() -> Result<ProjectDirs, Box<dyn std::error::Error>> {
    directories::ProjectDirs::from("", "", "license")
        .ok_or_else(|| "could not find project directories".into())
}

/// Directory for license files.
pub fn data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(proj_dirs()?.data_dir().into())
}

/// Returns all files within a directory.
/// 
/// Returns a list instead of an iterator because
/// we want to make sure there are no errors.
pub fn filenames<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    let files = dir.as_ref().read_dir()?
        .map(|rde| rde.map(|de| de.path()))
        .filter(|rp| rp.is_err() || rp.as_ref().unwrap().is_file())
        .collect::<io::Result<Vec<PathBuf>>>()?;
    Ok(files)
}

/// Wrapper around `fs::write` that first creates parent directories.
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fs::create_dir_all(path.as_ref().parent().unwrap())?;
    fs::write(path, contents)
}
