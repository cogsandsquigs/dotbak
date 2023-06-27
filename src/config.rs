use crate::errors::DotbackError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The configuration that Dotback uses to run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    include: Vec<String>,
}

impl Default for Config {
    /// The default configuration for Dotback.
    fn default() -> Self {
        Config { include: vec![] }
    }
}

// Helper functions for the configuration.

impl Config {
    /// Load the configuration from the given path.
    pub fn load_config_path<P>(path: P) -> Result<Self, DotbackError>
    where
        P: AsRef<Path>,
    {
        let config_str = std::fs::read_to_string(path)?;
        let config = toml::from_str(&config_str)?;

        Ok(config)
    }

    /// Load the configuration from the user's home directory.
    pub fn load_config() -> Result<Self, DotbackError> {
        let home_dir = dirs::home_dir().ok_or(DotbackError::NoHomeDir)?;
        let config_path = home_dir.join(".config/dotback/config.toml");

        Config::load_config_path(config_path)
    }
}
