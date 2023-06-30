pub mod config;
pub mod git;

use self::{config::ConfigError, git::GitError};
use miette::Diagnostic;
use std::io;
use thiserror::Error;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type Result<T> = std::result::Result<T, DotbakError>;

/// The main error type for the program itself. Note that the errors are wrapped in `Box`es to avoid
/// having rather large error types (thanks to `clippy` for pointing this out).
#[derive(Debug, Error, Diagnostic)]
pub enum DotbakError {
    /// An IO operations error occured.
    #[error("An IO operations error occured: {0}")]
    #[diagnostic(code(dotbak::error::io))]
    IO(
        #[source]
        #[from]
        io::Error, // No need to box this one, it's already boxed under the hood.
    ),

    /// A configuration error occured.
    #[error(transparent)]
    Config(#[from] Box<ConfigError>),

    /// A git error occured.
    #[error(transparent)]
    Git(#[from] Box<GitError>),

    /// There's no home directory for this computer.
    #[error("No home directory found for this computer! This should never happen!")]
    #[diagnostic(code(dotbak::error::no_home_dir))]
    NoHomeDir,
}

/* Convenience implementations for converting boxed errors into dotbak errors. */

/// Convert `ConfigError` into a `DotbakError`
impl From<ConfigError> for DotbakError {
    fn from(err: ConfigError) -> Self {
        Self::Config(Box::new(err))
    }
}

/// Convert `GitError` into a `DotbakError`
impl From<GitError> for DotbakError {
    fn from(err: GitError) -> Self {
        Self::Git(Box::new(err))
    }
}
