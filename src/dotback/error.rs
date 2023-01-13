use snafu::Snafu;
use std::io;
use toml;

/// All errors that can occur in the dotback library.
#[derive(Debug, Snafu)]
pub enum Error {
    /// A file could not be found.
    #[snafu(display("File does not exist: {}", source))]
    FileNotFound { source: io::Error },

    /// An error occurred while reading/writing to the configuration file. This does not apply to
    /// parsing errors, instead, it applies to errors that occur while reading/writing the file
    /// (like not being able to open the file, or not having permissions to read/write to the file).
    #[snafu(display("Error loading the configuration: {}", source))]
    ConfigLoading { source: io::Error },

    /// An error occured while parsing the configuration file.
    #[snafu(display("Error parsing the configuration: {}", source))]
    ConfigParsing { source: toml::de::Error },
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Self {
        match source.kind() {
            io::ErrorKind::NotFound => Error::FileNotFound { source },
            _ => Error::ConfigLoading { source },
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(source: toml::de::Error) -> Self {
        Error::ConfigParsing { source }
    }
}
