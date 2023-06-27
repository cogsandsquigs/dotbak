use miette::Diagnostic;
use thiserror::Error;

/// The main error type for the program itself.
#[derive(Debug, Error, Diagnostic)]
pub enum DotbackError {
    /// An IO operations error occured.
    #[error("IO error: {0}")]
    #[diagnostic(code(dotback::error::io))]
    IO(#[from] std::io::Error),

    /// A configuration parsing/deserialization error occured.
    #[error("Configuration deserialization error: {0}")]
    #[diagnostic(code(dotback::error::config::deserialize))]
    ConfigDeserialize(#[from] toml::de::Error),

    /// A configuration serialization error occured.
    #[error("Configuration serialization error: {0}")]
    #[diagnostic(code(dotback::error::config::serialize))]
    ConfigSerialize(#[from] toml::ser::Error),
}
