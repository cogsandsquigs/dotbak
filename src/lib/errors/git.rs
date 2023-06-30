use miette::Diagnostic;
use thiserror::Error;

use super::DotbakError;

/// Errors from doing Git operations.
#[derive(Debug, Error, Diagnostic)]
pub enum GitError {
    /// There was an error initializing the git repository.
    #[error("There was an error initializing the git repository.")]
    #[diagnostic(code(dotbak::error::git::init))]
    Init(#[from] gix::init::Error),
}

/* Convenience implementations for converting git errors into dotbak errors. */

/// Convert `GitError` into a `DotbakError`
impl From<GitError> for DotbakError {
    fn from(err: GitError) -> Self {
        Self::Git(Box::new(err))
    }
}
