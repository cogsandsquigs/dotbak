pub mod config;
pub mod error;

use config::Config;
use error::Error;
use std::{fs::File, io::Read};
use toml;

/// The default configuration path for dotback.
const DEFAULT_CONFIGURATION_PATH: &str = "~/.dotback/config.toml";

/// `Dotback` is sort of the "backend" for dotback. It manages dotfiles, configuration, and the
/// syncing process.
pub struct Dotback {
    /// The configuration for dotback.
    config: Config,
}

/// Public API for Dotback.
impl Dotback {
    /// Loads a `Dotback` instance from pre-existing configuration.
    /// Note that the configuration is loaded from the default location, `~/.dotback/config.toml`.
    pub fn load() -> Result<Self, Error> {
        let mut file = File::open(DEFAULT_CONFIGURATION_PATH)?;
        let mut contents = String::new();

        // Read the contents of `file` into `contents`.
        file.read_to_string(&mut contents)?;

        let config = toml::from_str(&contents)?;

        Ok(Dotback { config })
    }
}
