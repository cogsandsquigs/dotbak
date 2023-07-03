use miette::Diagnostic;
use snafu::prelude::*;
use std::{io, path::PathBuf};

/// The main error type for the program itself. Note that the errors are wrapped in `Box`es to avoid
/// having rather large error types (thanks to `clippy` for pointing this out).
#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub(crate)))]
pub enum IoError {
    /// A file or folder does not exist.
    #[snafu(display("File or folder '{}' does not exist", path.display()))]
    #[diagnostic(code(dotbak::error::io::not_found))]
    NotFound { path: PathBuf },

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

    /// An arbitrary command could not be run.
    #[snafu(display("Error running command '{} {}': {}", command, args.join(" "), source))]
    #[diagnostic(code(dotbak::error::git::arbitrary_command))]
    CommandIO {
        /// The command that was run.
        command: String,

        /// The arguments to the command.
        args: Vec<String>,

        /// The source io error.
        source: io::Error,
    },

    /// An arbitrary command was run and returned an error.
    #[snafu(display("Error running command '{} {}':\nSTDOUT:\n{}\nSTDERR:\n{}", command, args.join(" "), stdout, stderr))]
    #[diagnostic(code(dotbak::error::git::arbitrary_command))]
    CommandRun {
        /// The command that was run.
        command: String,

        /// The arguments to the command.
        args: Vec<String>,

        /// The stdout from the command.
        stdout: String,

        /// The stderr from the command.
        stderr: String,
    },
}
