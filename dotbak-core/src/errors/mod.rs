pub mod config;
pub mod io;

use self::{config::ConfigError, io::IoError};
use miette::Diagnostic;
use snafu::prelude::*;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type Result<T> = std::result::Result<T, DotbakError>;

/// The main error type for the program itself. This encapsulates all other errors, such as IO errors,
/// configuration errors, git errors, etc.
#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub(crate)))]
pub enum DotbakError {
    /// An IO operations error occured.
    Io { source: IoError },

    /// A configuration error occured.
    Config { source: ConfigError },

    // /// A git error occured.
    // Git { source: GitError },
    /// There's no home directory for this computer.
    #[snafu(display("No home directory found for this computer! This should never happen!"))]
    #[diagnostic(code(dotbak::error::no_home_dir))]
    NoHomeDir,
}

/* Convenience implementations for converting boxed errors into dotbak errors. */
/// Convert `IoError` into a `DotbakError`
impl From<IoError> for DotbakError {
    fn from(err: IoError) -> Self {
        Self::Io { source: err }
    }
}

/// Convert `ConfigError` into a `DotbakError`
impl From<ConfigError> for DotbakError {
    fn from(err: ConfigError) -> Self {
        Self::Config { source: err }
    }
}

// /// Convert `GitError` into a `DotbakError`
// impl From<GitError> for DotbakError {
//     fn from(err: GitError) -> Self {
//         Self::Git { source: err }
//     }
// }
