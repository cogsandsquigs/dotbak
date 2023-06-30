use miette::Diagnostic;
use thiserror::Error;

use super::DotbakError;

/// Errors from doing Git operations.
#[derive(Debug, Error, Diagnostic)]
pub enum GitError {
    /// There was an error initializing the git repository.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::git::init))]
    Init(#[from] gix::init::Error),

    /// There was an error cloning the git repository.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::git::clone))]
    Clone(#[from] gix::clone::Error),

    /// There was an error fetching the git repository.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::git::fetch))]
    Fetch(#[from] gix::clone::fetch::Error),

    /// There was an error with the main worktree.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::git::worktree))]
    Worktree(#[from] gix::clone::checkout::main_worktree::Error),

    /// There was an error finding the remote.
    #[error(transparent)]
    #[diagnostic(code(dotbak::error::git::find))]
    Find(#[from] gix::remote::find::existing::Error),
}

/* Convenience implementations for converting git errors into dotbak errors. */

/// Convert an init error into a `DotbakError`.
impl From<gix::init::Error> for DotbakError {
    fn from(err: gix::init::Error) -> Self {
        Self::Git(Box::new(GitError::Init(err)))
    }
}

/// Convert a clone error into a `DotbakError`.
impl From<gix::clone::Error> for DotbakError {
    fn from(err: gix::clone::Error) -> Self {
        Self::Git(Box::new(GitError::Clone(err)))
    }
}

/// Convert a fetch error into a `DotbakError`.
impl From<gix::clone::fetch::Error> for DotbakError {
    fn from(err: gix::clone::fetch::Error) -> Self {
        Self::Git(Box::new(GitError::Fetch(err)))
    }
}

/// Convert a worktree error into a `DotbakError`.
impl From<gix::clone::checkout::main_worktree::Error> for DotbakError {
    fn from(err: gix::clone::checkout::main_worktree::Error) -> Self {
        Self::Git(Box::new(GitError::Worktree(err)))
    }
}

/// Convert a find error into a `DotbakError`.
impl From<gix::remote::find::existing::Error> for DotbakError {
    fn from(err: gix::remote::find::existing::Error) -> Self {
        Self::Git(Box::new(GitError::Find(err)))
    }
}
