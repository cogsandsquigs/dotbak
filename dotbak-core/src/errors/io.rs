use miette::Diagnostic;
use snafu::prelude::*;
use std::{io, path::PathBuf};

/// The main error type for the program itself. Note that the errors are wrapped in `Box`es to avoid
/// having rather large error types (thanks to `clippy` for pointing this out).
#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub(crate)))]
pub enum IoError {
    /// A reading error: `std::io::Error`.
    #[snafu(display("Error reading from file '{}': {}", path.display(), source))]
    #[diagnostic(code(dotbak::error::io::read))]
    Read { source: io::Error, path: PathBuf },

    /// A writing error: `std::io::Error`.
    #[snafu(display("Error writing to file '{}': {}", path.display(), source))]
    #[diagnostic(code(dotbak::error::io::write))]
    Write { source: io::Error, path: PathBuf },

    /// A file creation error occured.
    #[snafu(display("Error creating file '{}': {}", path.display(), source))]
    #[diagnostic(code(dotbak::error::io::create))]
    Create { source: io::Error, path: PathBuf },

    /// A file deletion error occured.
    #[snafu(display("Error deleting file '{}': {}", path.display(), source))]
    #[diagnostic(code(dotbak::error::io::delete))]
    Delete { source: io::Error, path: PathBuf },
}
