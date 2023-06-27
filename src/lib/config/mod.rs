mod tests;

use crate::errors::DotbakError;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// The configuration that Dotbak uses to run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// The location of the configuration file.
    #[serde(skip)]
    pub path: PathBuf,

    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    pub include: Vec<String>,
}

impl Default for Config {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        Config {
            path: PathBuf::new(),
            include: vec![],
        }
    }
}

// Helper functions for the configuration.

impl Config {
    /// Load the configuration from the user's home directory. Note that this uses
    /// `AppData\Local` on windows instead of `AppData\Roaming` (although Windows is
    /// not a target platform for this application).
    pub fn load_config() -> Result<Self, DotbakError> {
        Config::load_config_path(Config::config_path())
    }
}

/// Private API for the configuration.
impl Config {
    /// Gets the directory of the configuration file. Note that this uses `AppData\Local`
    /// on windows instead of `AppData\Roaming` (although Windows is not a target platform
    /// for this application).
    fn config_dir() -> PathBuf {
        // We use `config_local_dir` instead of `config_dir` because AppData\Local makes
        // more sense for this application on windows (even though it's not a target). For
        // other platforms, it's the same as `config_dir`.
        let config_dir = dirs::config_local_dir().unwrap();

        config_dir.join("dotbak")
    }

    /// Gets the location of the configuration file. Note that this uses `AppData\Local`
    /// on windows instead of `AppData\Roaming` (although Windows is not a target platform
    /// for this application).
    fn config_path() -> PathBuf {
        // We use `config_local_dir` instead of `config_dir` because AppData\Local makes
        // more sense for this application on windows (even though it's not a target). For
        // other platforms, it's the same as `config_dir`.
        let config_dir = dirs::config_local_dir().unwrap();

        config_dir.join("dotbak/config.toml")
    }

    /// Load the configuration from the given path. If the configuration file or folder does not exist,
    /// the default configuration is created at that location and returned.
    fn load_config_path<P>(path: P) -> Result<Self, DotbakError>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut config: Config;

        // If the path doesn't exist, create the directory and the default configuration.
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;

            config = Config::default();

            // Get the config string and write it to the new file.
            let config_str = toml::to_string_pretty(&config)?;
            fs::write(path, config_str)?;
        }
        // Otherwise, load the configuration from the file.
        else {
            let config_str = fs::read_to_string(path)?;
            config = toml::from_str(&config_str)?;
        }

        config.path = path.to_path_buf();

        Ok(config)
    }
}
