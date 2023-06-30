pub mod config;
pub mod errors;
pub mod git;
pub mod locations;

use config::Config;
use errors::{config::ConfigError, DotbakError, Result};
use locations::CONFIG_PATH;

// #[cfg(test)]
// use std::path::Path;

/// The main structure to manage the program's actions and such.
pub struct Dotbak {
    /// The configuration for the program.
    pub config: Config,
}

/// Public API for the program.
impl Dotbak {
    /// Create a new instance of the program. If the configuration file does not exist, it will be created.
    /// If it does exist, it will be loaded.
    pub fn init() -> Result<Self> {
        // Try to load the configuration file.
        match Config::load_config(&CONFIG_PATH) {
            // If the configuration file exists, load it.
            // TODO: log that the configuration file was loaded, not created.
            Ok(config) => Ok(Dotbak { config }),

            // If the configuration file does not exist, create it.
            // TODO: log that the configuration file was created, not loaded.
            Err(DotbakError::Config(err)) => {
                // Have to dereference the error to check if it's a `ConfigNotFound` error.
                if let ConfigError::ConfigNotFound(_) = *err {
                    let config = Config::load_config(&CONFIG_PATH)?;

                    Ok(Dotbak { config })
                } else {
                    Err(DotbakError::Config(err))
                }
            }

            // If the error is not a `ConfigNotFound` error, return it.
            Err(err) => Err(err),
        }
    }

    /// Creates a new instance of the program. If the configuration file does not exist, an error will be returned.
    /// If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        let config: Config = Config::load_config(&CONFIG_PATH)?;

        Ok(Dotbak { config })
    }
}

/// Public testing API for the program.
#[cfg(test)]
impl Dotbak {}
