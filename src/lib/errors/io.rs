use miette::Diagnostic;
use std::{io, path::PathBuf};
use thiserror::Error;

// /// The main error type for the program itself. Note that the errors are wrapped in `Box`es to avoid
// /// having rather large error types (thanks to `clippy` for pointing this out).
// #[derive(Debug, Snafu, Diagnostic)]
// #[snafu(visibility(pub(crate)))]
// pub enum IoError {
//     /// A file or folder does not exist.
//     #[snafu(display("File or folder '{}' does not exist", path.display()))]
//     #[diagnostic(code(dotbak::error::io::not_found))]
//     NotFound {
//         /// The path to the file/folder that does not exist.
//         path: PathBuf,
//     },

//     /// A reading error: `std::io::Error`.
//     #[snafu(display("Error reading from file or folder '{}': {}", path.display(), source))]
//     #[diagnostic(code(dotbak::error::io::read))]
//     Read {
//         /// The path to the file being read from.
//         path: PathBuf,
//         /// The source io error.
//         source: io::Error,
//     },

//     /// A writing error: `std::io::Error`.
//     #[snafu(display("Error writing to file '{}': {}", path.display(), source))]
//     #[diagnostic(code(dotbak::error::io::write))]
//     Write {
//         /// The path to the file being written to.
//         path: PathBuf,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// A file/folder creation error occured.
//     #[snafu(display("Error creating file or folder '{}': {}", path.display(), source))]
//     #[diagnostic(code(dotbak::error::io::create))]
//     Create {
//         /// The path to the file/folder being created.
//         path: PathBuf,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// An error moving a file/folder occured.
//     #[snafu(display("Error moving file or folder '{}' to '{}': {}", from.display(), to.display(), source))]
//     #[diagnostic(code(dotbak::error::io::moving))]
//     Move {
//         /// The path to the file/folder being moved.
//         from: PathBuf,

//         /// The path to the destination.
//         to: PathBuf,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// A symlink creation error occured.
//     #[snafu(display("Error creating symlink from '{}' to '{}': {}", from.display(), to.display(), source))]
//     #[diagnostic(code(dotbak::error::io::symlink))]
//     Symlink {
//         /// The path to the file/folder being symlinked.
//         from: PathBuf,

//         /// The path to the symlink.
//         to: PathBuf,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// A file deletion error occured.
//     #[snafu(display("Error deleting file '{}': {}", path.display(), source))]
//     #[diagnostic(code(dotbak::error::io::delete))]
//     Delete {
//         /// The path to the file/folder being deleted.
//         path: PathBuf,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// An arbitrary command could not be run.
//     #[snafu(display("Error running command '{} {}': {}", command, args.join(" "), source))]
//     #[diagnostic(code(dotbak::error::git::arbitrary_command))]
//     CommandIO {
//         /// The command that was run.
//         command: String,

//         /// The arguments to the command.
//         args: Vec<String>,

//         /// The source io error.
//         source: io::Error,
//     },

//     /// An arbitrary command was run and returned an error.
//     #[snafu(display("Error running command '{} {}':\n{}{}", command, args.join(" "), stdout, stderr))]
//     #[diagnostic(code(dotbak::error::git::arbitrary_command))]
//     CommandRun {
//         /// The command that was run.
//         command: String,

//         /// The arguments to the command.
//         args: Vec<String>,

//         /// The stdout from the command.
//         stdout: String,

//         /// The stderr from the command.
//         stderr: String,
//     },
// }

// Convert the above SNAFU enum and implications into a Thiserror enum.

#[derive(Debug, Error, Diagnostic)]
pub enum IoError {
    /// A file or folder does not exist.
    #[error("File or folder '{path}' does not exist")]
    #[diagnostic(code(dotbak::error::io::not_found))]
    NotFound {
        /// The path to the file/folder that does not exist.
        path: PathBuf,
    },

    /// A reading error: `std::io::Error`.
    #[error("Error reading from file or folder '{path}': {source}")]
    #[diagnostic(code(dotbak::error::io::read))]
    Read {
        /// The path to the file being read from.
        path: PathBuf,
        /// The source io error.
        source: io::Error,
    },

    /// A writing error: `std::io::Error`.
    #[error("Error writing to file '{path}': {source}")]
    #[diagnostic(code(dotbak::error::io::write))]
    Write {
        /// The path to the file being written to.
        path: PathBuf,

        /// The source io error.
        source: io::Error,
    },

    /// A file/folder creation error occured.
    #[error("Error creating file or folder '{path}': {source}")]
    #[diagnostic(code(dotbak::error::io::create))]
    Create {
        /// The path to the file/folder being created.
        path: PathBuf,

        /// The source io error.
        source: io::Error,
    },

    /// An error moving a file/folder occured.
    #[error("Error moving file or folder '{from}' to '{to}': {source}")]
    #[diagnostic(code(dotbak::error::io::moving))]
    Move {
        /// The path to the file/folder being moved.
        from: PathBuf,

        /// The path to the destination.
        to: PathBuf,

        /// The source io error.
        source: io::Error,
    },

    /// A symlink creation error occured.
    #[error("Error creating symlink from '{from}' to '{to}': {source}")]
    #[diagnostic(code(dotbak::error::io::symlink))]
    Symlink {
        /// The path to the file/folder being symlinked.
        from: PathBuf,

        /// The path to the symlink.
        to: PathBuf,

        /// The source io error.
        source: io::Error,
    },

    /// A file deletion error occured.
    #[error("Error deleting file '{path}': {source}")]
    #[diagnostic(code(dotbak::error::io::delete))]
    Delete {
        /// The path to the file/folder being deleted.
        path: PathBuf,

        /// The source io error.
        source: io::Error,
    },

    /// An arbitrary command could not be run.
    #[error("Error running command '{command} {}': {source}", args.join(" "))]
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
    #[error("Error running command '{command} {}':\n{stdout}{stderr}", args.join(" "))]
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
