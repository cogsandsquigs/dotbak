mod tests;

use crate::errors::{DotbakError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};

// #[cfg(test)]
// use std::path::Path;

/// The configuration that Dotbak uses to run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// The location of the configuration file.
    #[serde(skip)]
    pub path: PathBuf,

    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    #[serde(default)]
    pub include: Vec<String>,

    /// The exclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    #[serde(default)]
    pub exclude: Vec<String>,
}

impl Default for Config {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        Config {
            path: PathBuf::new(),
            include: vec![],
            exclude: vec![],
        }
    }
}

/// Public API for the configuration.
impl Config {
    /// Load the configuration from the user's home directory. Note that this uses
    /// `AppData\Local` on windows instead of `AppData\Roaming` (although Windows is
    /// not a target platform for this application).
    pub fn load_config() -> Result<Self> {
        let config_path = Self::config_path()?;
        let config = Self::get_config(config_path)?;

        Ok(config)
    }

    /// Gets the location of the configuration file.
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::dotbak_dir()?.join("config.toml"))
    }

    /// Gets the location of the git repository where the backups are stored.
    pub fn repo_path() -> Result<PathBuf> {
        Ok(Self::dotbak_dir()?.join("repo"))
    }

    /// Gets the directory of the `.dotbak` directory.
    pub fn dotbak_dir() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .ok_or(DotbakError::NoHomeDir)?
            .join(".dotbak");

        Ok(config_dir)
    }
}

/// Public testing API for the configuration.
impl Config {
    /// Load the configuration from the given path to the parent of the `.dotbak` directory. For
    /// example, if the path is `/home/some_user`, then the configuration will be loaded from
    /// `/home/some_user/.dotbak/config.toml`. If that path doesn't exist, then the directory
    /// will be created and the default configuration will be written to the file.
    #[cfg(test)]
    pub fn load_config_path<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let config_path = path.join(".dotbak/config.toml");
        let config = Self::get_config(config_path)?;

        Ok(config)
    }
}

/// Private API for the configuration.
impl Config {
    /// Gets the config from a given path. If the path doesn't exist, then the directory will be
    /// created and the default configuration will be written to the file.
    fn get_config<P>(config_path: P) -> Result<Config>
    where
        P: AsRef<Path>,
    {
        let config_path = config_path.as_ref();
        let mut config: Config;

        // If the path doesn't exist, create the directory and the default configuration.
        if !config_path.exists() {
            fs::create_dir_all(config_path.parent().expect("This should never happen."))?;

            config = Self::default();

            // Get the config string and write it to the new file.
            let config_str = toml::to_string_pretty(&config)?;
            fs::write(config_path, config_str)?;
        }
        // Otherwise, load the configuration from the file.
        else {
            let config_str = fs::read_to_string(config_path)?;
            config = toml::from_str(&config_str)?;
        }

        config.path = config_path.to_path_buf();

        Ok(config)
    }
}
