pub mod config;
pub mod errors;
pub mod git;
pub mod locations;

use config::Config;
use errors::Result;
use locations::CONFIG_PATH;

#[cfg(test)]
use std::path::Path;

/// The main structure to manage the program's actions and such.
pub struct Dotbak {
    /// The configuration for the program.
    pub config: Config,
}

/// Public API for the program.
impl Dotbak {
    /// Create a new instance of the program. This also automatically loads the configuration.
    pub fn init() -> Result<Self> {
        let config: Config = Config::load_config(&CONFIG_PATH)?;

        Ok(Dotbak { config })
    }
}

/// Public testing API for the program.
#[cfg(test)]
impl Dotbak {
    /// Create a new instance of the program with a custom configuration path.
    /// This is mostly used for testing.
    pub fn new_with_config_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let config = Config::load_config(config_path)?;

        Ok(Dotbak { config })
    }

    /// Create a new instance of the program with a custom configuration.
    /// This is mostly used for testing.
    pub fn new_with_config(config: Config) -> Self {
        Dotbak { config }
    }
}
