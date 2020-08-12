//! Provides information on popular licenses and ways to
//! download the license text files locally.
//! 
//! License text files can be found here:
//! https://github.com/spdx/license-list-data/tree/master/text

pub mod dirs;
pub mod net;

use std::path::{Path, PathBuf};

/// Information on a license.
pub struct License {
    /// Short name or SPDX identifier
    short: String,
    /// Longer, more descriptive name
    long: String,
    /// URL of license text
    url: String,
    /// Path to the license file
    path: PathBuf,
}

impl License {
    /// Gets license information from the SPDX identifier.
    /// 
    /// Forwards error code from creating the path.
    pub fn from(spdx: &str) -> Result<License, Box<dyn std::error::Error>> {
        let default = License::default(spdx)?;
        let license = match spdx {
            "MIT" => License {
                long: "MIT License".into(),
                ..default
            },
            "Apache-2.0" => License {
                long: "Apache License 2.0".into(),
                url: String::from("https://www.apache.org/licenses/LICENSE-2.0.txt"),
                ..default
            },
            _ => default,
        };
        Ok(license)
    }

    /// Returns default license values for any SPDX identifier.
    fn default(spdx: &str) -> Result<License, Box<dyn std::error::Error>> {
        Ok(License {
            short: String::from(spdx),
            long: String::from(spdx),
            url: License::default_url(spdx),
            path: License::build_path(spdx)?,
        })
    }

    /// Returns a default URL that will probably have the license file.
    ///
    /// Note that the formatting might be a bit off. For example,
    /// the spacing is different for Apache-2.0.
    fn default_url(spdx: &str) -> String {
        static URL_TEMPLATE: &str =
            "https://raw.githubusercontent.com/spdx/license-list-data/master/text/SPDX.txt";

        URL_TEMPLATE.replace("SPDX", spdx)
    }

    /// Builds the path to a license file.
    ///
    /// Does not guarantee that the file exists.
    fn build_path(spdx: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut dir = dirs::data_dir()?;
        dir.push(spdx);
        dir.set_extension("txt");
        Ok(dir)
    }

    /// Returns short name of the license.
    pub fn short(&self) -> &str {
        &self.short
    }

    /// Returns long name of the license.
    pub fn long(&self) -> &str {
        &self.long
    }

    /// Returns a URL to the license text.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns path to the license file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}
