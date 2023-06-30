use super::DotbakError;
use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

/// All errors related to the configuration file.
#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    /// A configuration parsing/deserialization error occured.
    #[error("Error deserializing the configuration: {0}")]
    #[diagnostic(code(dotbak::error::config::deserialize))]
    ConfigDeserialize(#[from] toml::de::Error),

    /// A configuration serialization error occured.
    #[error("Error serializing the configuration: {0}")]
    #[diagnostic(code(dotbak::error::config::serialize))]
    ConfigSerialize(#[from] toml::ser::Error),

    /// Configuration file not found.
    #[error("The configuration file '{0}' does not exist!")]
    #[diagnostic(code(dotbak::error::config::not_found))]
    ConfigNotFound(PathBuf),

    /// The configuration file already exists.
    #[error("The configuration file '{0}' already exists!")]
    #[diagnostic(code(dotbak::error::config::already_exists))]
    ConfigAlreadyExists(PathBuf),
}

/* Convenience implementations for converting toml ser/de errors into config errors. */

/// Convert `toml::de::Error` into a `DotbakError`
impl From<toml::de::Error> for DotbakError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(ConfigError::ConfigDeserialize(err))
    }
}

/// Convert `toml::ser::Error` into a `DotbakError`
impl From<toml::ser::Error> for DotbakError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Config(ConfigError::ConfigSerialize(err))
    }
}
