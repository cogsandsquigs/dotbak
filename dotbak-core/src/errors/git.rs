use miette::Diagnostic;
use snafu::Snafu;
use std::path::PathBuf;

// use super::DotbakError;

/// Errors from doing Git operations.
#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub(crate)))]
pub enum GitError {
    /// There was an error initializing the git repository.
    #[snafu(display("Error initializing git repository{} at {}: {}", url.as_ref().map(|url| format!(" '{}'", url)).unwrap_or("".into()), path.display(), source))]
    #[diagnostic(code(dotbak::error::git::init))]
    Init {
        /// The URL of the repository. Optional because it may not be set.
        url: Option<String>,

        /// The location of the repository.
        path: PathBuf,

        /// The source git error.
        source: git2::Error,
    },

    /// There was an error cloning the git repository.
    #[snafu(display("Error cloning git repository '{}' to {}: {}", url, path.display(), source))]
    #[diagnostic(code(dotbak::error::git::clone))]
    Clone {
        /// The URL of the repository. Optional because it may not be set.
        url: String,

        /// The path to the repository.
        path: PathBuf,

        /// The source git error.
        source: git2::Error,
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
