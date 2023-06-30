use miette::Diagnostic;
use snafu::Snafu;

// use super::DotbakError;

/// Errors from doing Git operations.
#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub(crate)))]
pub enum GitError {
    /// There was an error initializing the git repository.
    #[snafu(display("Error initializing the git repository: {}", source))]
    #[diagnostic(code(dotbak::error::git::init))]
    Init { source: gix::init::Error },

    /// There was an error cloning the git repository.
    #[snafu(display("Error cloning git repository '{}': {}", url.to_bstring(), source))]
    #[diagnostic(code(dotbak::error::git::clone))]
    Clone {
        source: gix::clone::Error,
        url: gix::url::Url,
    },

    /// There was an error fetching the git repository.
    #[snafu(display("Error fetching '{}': {}", url.to_bstring(), source))]
    #[diagnostic(code(dotbak::error::git::fetch))]
    Fetch {
        source: gix::clone::fetch::Error,
        url: gix::url::Url,
    },

    /// There was an error with the main worktree.
    #[snafu(display("Error with the main worktree: {}", source))]
    #[diagnostic(code(dotbak::error::git::worktree))]
    Worktree {
        source: gix::clone::checkout::main_worktree::Error,
    },

    /// There was an error finding the remote.
    #[snafu(display("Error finding the remote '{}': {}", url.to_bstring(), source))]
    #[diagnostic(code(dotbak::error::git::find))]
    Find {
        source: gix::remote::find::existing::Error,
        url: gix::url::Url,
    },
}

// /* Convenience implementations for converting git errors into dotbak errors. */
// /// Convert an init error into a `DotbakError`.
// impl From<gix::init::Error> for DotbakError {
//     fn from(err: gix::init::Error) -> Self {
//         Self::Git(Box::new(GitError::Init(err)))
//     }
// }

// /// Convert a clone error into a `DotbakError`.
// impl From<gix::clone::Error> for DotbakError {
//     fn from(err: gix::clone::Error) -> Self {
//         Self::Git(Box::new(GitError::Clone(err)))
//     }
// }

// /// Convert a fetch error into a `DotbakError`.
// impl From<gix::clone::fetch::Error> for DotbakError {
//     fn from(err: gix::clone::fetch::Error) -> Self {
//         Self::Git(Box::new(GitError::Fetch(err)))
//     }
// }

// /// Convert a worktree error into a `DotbakError`.
// impl From<gix::clone::checkout::main_worktree::Error> for DotbakError {
//     fn from(err: gix::clone::checkout::main_worktree::Error) -> Self {
//         Self::Git(Box::new(GitError::Worktree(err)))
//     }
// }

// /// Convert a find error into a `DotbakError`.
// impl From<gix::remote::find::existing::Error> for DotbakError {
//     fn from(err: gix::remote::find::existing::Error) -> Self {
//         Self::Git(Box::new(GitError::Find(err)))
//     }
// }
