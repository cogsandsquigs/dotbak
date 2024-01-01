use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

use super::DotbakError;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

// /// All errors related to the configuration file.
// #[derive(Debug, Snafu, Diagnostic)]
// pub enum ConfigError {
//     /// A configuration parsing/deserialization error occured.
//     #[snafu(display("Error deserializing the configuration: {source}"))]
//     #[diagnostic(code(dotbak::error::config::deserialize))]
//     ConfigDeserialize { source: toml::de::Error },

//     /// A configuration serialization error occured.
//     #[snafu(display("Error serializing the configuration: {source}"))]
//     #[diagnostic(code(dotbak::error::config::serialize))]
//     ConfigSerialize { source: toml::ser::Error },

//     /// Configuration file not found.
//     #[snafu(display("The configuration file '{}' does not exist!", path.display()))]
//     #[diagnostic(code(dotbak::error::config::not_found))]
//     ConfigNotFound { path: PathBuf },

//     /// The configuration file already exists.
//     #[snafu(display("The configuration file '{}' already exists!", path.display()))]
//     #[diagnostic(code(dotbak::error::config::already_exists))]
//     ConfigAlreadyExists { path: PathBuf },
// }

// /* Convenience implementations for converting toml ser/de errors into dotbak errors. */
// /// Convert `toml::de::Error` into a `DotbakError`
// impl From<toml::de::Error> for DotbakError {
//     fn from(err: toml::de::Error) -> Self {
//         Self::Config {
//             source: ConfigError::ConfigDeserialize { source: err },
//         }
//     }
// }

// /// Convert `toml::ser::Error` into a `DotbakError`
// impl From<toml::ser::Error> for DotbakError {
//     fn from(err: toml::ser::Error) -> Self {
//         Self::Config {
//             source: ConfigError::ConfigSerialize { source: err },
//         }
//     }
// }

// Convert the above SNAFU enum and implications into a Thiserror enum.

#[derive(Debug, Error, Diagnostic)]
pub enum ConfigError {
    /// A configuration parsing/deserialization error occured.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::config::deserialize))]
    ConfigDeserialize { source: toml::de::Error },

    /// A configuration serialization error occured.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::config::serialize))]
    ConfigSerialize { source: toml::ser::Error },

    /// Configuration file not found.
    #[error("The configuration file '{path}' does not exist!")]
    #[diagnostic(code(dotbak::error::config::not_found))]
    ConfigNotFound { path: PathBuf },

    /// The configuration file already exists.
    #[error("The configuration file '{path}' already exists!")]
    #[diagnostic(code(dotbak::error::config::already_exists))]
    ConfigAlreadyExists { path: PathBuf },
}

/* Convenience implementations for converting toml ser/de errors into dotbak errors. */
/// Convert `toml::de::Error` into a `DotbakError`
impl From<toml::de::Error> for DotbakError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(ConfigError::ConfigDeserialize { source: err })
    }
}

/// Convert `toml::ser::Error` into a `DotbakError`
impl From<toml::ser::Error> for DotbakError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Config(ConfigError::ConfigSerialize { source: err })
    }
}
