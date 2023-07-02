use miette::Diagnostic;
use snafu::Snafu;
use std::path::PathBuf;

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
