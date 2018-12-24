#![warn(clippy::all)]
#![warn(clippy::pedantic)]
mod config;
mod manifest;
pub use self::{config::*, manifest::*};

use serde::de::DeserializeOwned;
use std::{io::Error as IoError, path::Path, result::Result};
use toml::de::Error;

#[derive(Debug)]
pub struct CargoTomlError {
    inner: ErrorKind,
}

impl std::fmt::Display for CargoTomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::error::Error for CargoTomlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.inner {
            ErrorKind::Parse(e) => Some(e),
            ErrorKind::Io(e) => Some(e),
        }
    }
}

impl From<Error> for CargoTomlError {
    fn from(e: Error) -> Self {
        Self {
            inner: ErrorKind::Parse(e),
        }
    }
}

impl From<IoError> for CargoTomlError {
    fn from(e: IoError) -> Self {
        Self {
            inner: ErrorKind::Io(e),
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Parse(Error),
    Io(IoError),
}

/// Reads the file at `path`.
///
/// # Examples
///
/// ```rust
/// # use cargo_toml2::CargoToml;
/// # use cargo_toml2::from_path;
/// // Reading a CargoToml
/// let toml: CargoToml = from_path("Cargo.toml").expect("Failed to read Cargo.toml");
/// ```
///
/// # Errors
///
/// If reading the provided `path` fails, or deserialization fails.
pub fn from_path<T: AsRef<Path>, R: DeserializeOwned>(path: T) -> Result<R, CargoTomlError> {
    let path = path.as_ref();
    let toml = std::fs::read_to_string(path)?;
    let x: R = toml::from_str(&toml)?;
    Ok(x)
}
