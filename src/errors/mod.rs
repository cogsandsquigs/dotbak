pub mod config;
pub mod io;

use self::{config::ConfigError, io::IoError};
use miette::Diagnostic;
use thiserror::Error;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type Result<T> = std::result::Result<T, DotbakError>;

/// The main error type for the program itself. This encapsulates all other errors, such as IO errors,
/// configuration errors, git errors, etc.
#[derive(Debug, Error, Diagnostic)]
pub enum DotbakError {
    /// An IO operations error occured.
    #[error(transparent)]
    Io(#[from] IoError),

    /// A configuration error occured.
    #[error(transparent)]
    Config(#[from] ConfigError),
}

// /* Convenience implementations for converting boxed errors into dotbak errors. */
// /// Convert `IoError` into a `DotbakError`
// impl From<IoError> for DotbakError {
//     fn from(err: IoError) -> Self {
//         Self::Io { source: err }
//     }
// }

// /// Convert `ConfigError` into a `DotbakError`
// impl From<ConfigError> for DotbakError {
//     fn from(err: ConfigError) -> Self {
//         Self::Config { source: err }
//     }
// }
