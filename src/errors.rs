use miette::Diagnostic;
use thiserror::Error;

/// The main error type for the program itself.
#[derive(Debug, Error, Diagnostic)]
pub enum DotbackError {
    /// An IO operations error occured.
    #[error("IO error: {0}")]
    #[diagnostic(code(dotback::error::io))]
    IO(#[from] std::io::Error),

    /// A configuration error occured.
    #[error("Configuration error: {0}")]
    #[diagnostic(code(dotback::error::config))]
    Config(#[from] confy::ConfyError),
}
