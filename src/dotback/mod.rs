pub mod config;
pub mod error;

use config::Config;
use error::Error;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

/// The default configuration path for dotback.
const DEFAULT_CONFIGURATION_PATH: &str = "~/.dotback/config.toml";

/// `Dotback` is sort of the "backend" for dotback. It manages dotfiles, configuration, and the
/// syncing process.
pub struct Dotback {
    /// The path to the configuration file.
    config_path: PathBuf,

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

        Ok(Dotback {
            config,
            config_path: PathBuf::from(DEFAULT_CONFIGURATION_PATH),
        })
    }

    /// Writes the `Config` instance to a configuration file.
    /// Note that the configuration is stored to the default location, `~/.dotback/config.toml`.
    pub fn write_config(&self) -> Result<(), Error> {
        let content = toml::to_vec(&self.config)?;

        let mut file = File::create(&self.config_path)?;

        // Write the contents of `content` to `file`.
        file.write_all(&content)?;

        Ok(())
    }

    /// Reads the configuration file and returns the `Config` instance.
    /// Note that the configuration is read from the default location, `~/.dotback/config.toml`.
    pub fn read_config(&self) -> Result<Config, Error> {
        let mut file = File::open(&self.config_path)?;
        let mut contents = Vec::new();

        // Read all of the contents of `file` into `contents`.
        file.read_to_end(&mut contents)?;

        let config = toml::from_slice(&contents)?;

        Ok(config)
    }
}
