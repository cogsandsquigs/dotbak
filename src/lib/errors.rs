use miette::Diagnostic;
use std::{io, path::PathBuf};
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

    /// Configuration file not found.
    #[error("The configuration file is not found: {0}")]
    #[diagnostic(code(dotbak::error::config::not_found))]
    ConfigNotFound(PathBuf),

    /// A configuration parsing/deserialization error occured.
    #[error("Configuration deserialization error: {0}")]
    #[diagnostic(code(dotbak::error::config::deserialize))]
    ConfigDeserialize(#[from] toml::de::Error),

    /// A configuration serialization error occured.
    #[error("Configuration serialization error: {0}")]
    #[diagnostic(code(dotbak::error::config::serialize))]
    ConfigSerialize(#[from] toml::ser::Error),

    /// There is already a git repository initialized.
    #[error("There is already a git repository initialized.")]
    #[diagnostic(code(dotbak::error::git::already_initialized))]
    GitAlreadyInitialized,
}
