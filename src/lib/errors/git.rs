use miette::Diagnostic;
use thiserror::Error;

/// Errors from doing Git operations.
#[derive(Debug, Error, Diagnostic)]
pub enum GitError {
    /// There is already a git repository initialized.
    #[error("There is already a git repository initialized.")]
    #[diagnostic(code(dotbak::error::git::already_initialized))]
    GitAlreadyInitialized,
}
