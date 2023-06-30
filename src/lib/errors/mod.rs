pub mod config;
pub mod git;

use self::config::ConfigError;
use miette::Diagnostic;
use std::io;
use thiserror::Error;

/// A helper return type for functions that return `Result<T, DotbakError>`.
pub type Result<T> = std::result::Result<T, DotbakError>;

/// The main error type for the program itself.
#[derive(Debug, Error, Diagnostic)]
pub enum DotbakError {
    /// An IO operations error occured.
    #[error("IO error: {0}")]
    #[diagnostic(code(dotbak::error::io))]
    IO(#[from] io::Error),

    /// A configuration error occured.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// There is already a git repository initialized.
    #[error("There is already a git repository initialized.")]
    #[diagnostic(code(dotbak::error::git::already_initialized))]
    GitAlreadyInitialized,
}
