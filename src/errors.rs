use miette::Diagnostic;
use thiserror::Error;

/// The main error type for the program itself.
#[derive(Debug, Error, Diagnostic)]
pub enum DotbakError {
    /// An IO operations error occured.
    #[error("IO error: {0}")]
    #[diagnostic(code(dotbak::error::io))]
    IO(#[from] std::io::Error),

    /// A configuration parsing/deserialization error occured.
    #[error("Configuration deserialization error: {0}")]
    #[diagnostic(code(dotbak::error::config::deserialize))]
    ConfigDeserialize(#[from] toml::de::Error),

    /// A configuration serialization error occured.
    #[error("Configuration serialization error: {0}")]
    #[diagnostic(code(dotbak::error::config::serialize))]
    ConfigSerialize(#[from] toml::ser::Error),

    /// No home directory was found.
    #[error("No home directory found for this computer.")]
    #[diagnostic(
        code(dotbak::error::no_home_dir),
        help("This error should rarely happen, if at all. Somehow set your home directory for this computer?\nRelevant documentation for this error: https://docs.rs/dirs/latest/dirs/fn.config_local_dir.html")
    )]
    NoHomeDir,
}
