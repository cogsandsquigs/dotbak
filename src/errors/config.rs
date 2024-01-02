use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

use super::DotbakError;

#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    /// A configuration parsing/deserialization error occured.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::config::deserialize))]
    Deserialize { source: toml::de::Error },

    /// A configuration serialization error occured.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::config::serialize))]
    Serialize { source: toml::ser::Error },

    /// Configuration file not found.
    #[error("The configuration file '{path}' does not exist!")]
    #[diagnostic(code(dotbak::error::config::not_found))]
    NotFound { path: PathBuf },

    /// The configuration file already exists.
    #[error("The configuration file '{path}' already exists!")]
    #[diagnostic(code(dotbak::error::config::already_exists))]
    AlreadyExists { path: PathBuf },
}

/* Convenience implementations for converting toml ser/de errors into dotbak errors. */
/// Convert `toml::de::Error` into a `DotbakError`
impl From<toml::de::Error> for DotbakError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(ConfigError::Deserialize { source: err })
    }
}

/// Convert `toml::ser::Error` into a `DotbakError`
impl From<toml::ser::Error> for DotbakError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Config(ConfigError::Serialize { source: err })
    }
}
