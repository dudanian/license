//! Net helper functions.
//! 
//! Really just wrapping around multiple implementations
//! of a file GET.

#[doc(inline)]
pub use get_backend::download;

#[cfg(feature = "curl-get")]
mod get_backend {
    use curl::easy::Easy;

    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    /// Download a file from the given URL and save it in the given file.
    pub fn download<U: AsRef<str>, P: AsRef<Path>>(
        url: U,
        file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut easy = Easy::new();
        let mut file = File::create(file)?;

        easy.url(url.as_ref())?;
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            // I think the best way to indicate error
            // is to return a 0 length
            file.write(data).or(Ok(0))
        })?;
        transfer.perform()?;
        Ok(())
    }
}

#[cfg(feature = "reqwest-get")]
mod get_backend {
    use reqwest::blocking as reqwest;

    use std::path::Path;

    /// Download a file from the given URL and save it in the given file.
    pub fn download<U: AsRef<str>, P: AsRef<Path>>(
        url: U,
        file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(url.as_ref())?;
        let contents = response.text()?;
        crate::dirs::write(file, contents)?;
        Ok(())
    }
}

#[cfg(feature = "ureq-get")]
mod get_backend {
    use ureq;

    use std::path::Path;
    use std::time::Duration;

    /// Download a file from the given URL and save it in the given file.
    pub fn download<U: AsRef<str>, P: AsRef<Path>>(
        url: U,
        file: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = ureq::get(url.as_ref())
            .timeout(Duration::from_secs(5))
            .call();
        if response.error() {
            return Err("failed to download file".into());
        }
        crate::dirs::write(file, response.into_string()?)?;
        Ok(())
    }
}
