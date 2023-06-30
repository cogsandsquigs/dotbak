mod tests;

use crate::errors::{config::ConfigError, Result};
use crate::locations::{CONFIG_FILE_NAME, REPO_FOLDER_NAME};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};

/// The configuration that Dotbak uses to run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {
    /// The location of the configuration file. This is a temporary value that will be overwritten
    /// later when loading in `Config::load_config`, so it is not serialized.
    #[serde(skip)]
    pub path: PathBuf,

    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/config.toml"]`, which is the configuration file itself.
    #[serde(default = "Config::default_include")]
    pub include: Vec<String>,

    /// The exclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/repo"]`, which is the configuration file itself.
    #[serde(default = "Config::default_exclude")]
    pub exclude: Vec<String>,
}

impl Default for Config {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        Config {
            path: PathBuf::new(), // This is a temporary value that will be overwritten later.
            include: Config::default_include(),
            exclude: Config::default_exclude(),
        }
    }
}

/// Public API for the configuration.
impl Config {
    /// Loads the config file from the given path. If the path doesn't exist, it will return an error.
    pub fn load_config<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut config: Config;

        if !path.exists() {
            return Err(ConfigError::ConfigNotFound(path.to_path_buf()).into());
        }

        let config_str = fs::read_to_string(path)?;
        config = toml::from_str(&config_str)?;

        // IMPORTANT: This is the only place where the path is set.
        config.path = path.to_path_buf();

        Ok(config)
    }

    /// Saves the config file to the given path. If the path doesn't exist, it will return an error.
    pub fn save_config<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::ConfigNotFound(path.to_path_buf()).into());
        }

        let config_str = toml::to_string(self)?;
        fs::write(path, config_str)?;

        Ok(())
    }

    /// Creates a new config file at the given path. If the path already exists, it will return an error.
    pub fn create_config<P>(path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path: &Path = path.as_ref();

        if path.exists() {
            return Err(ConfigError::ConfigAlreadyExists(path.to_path_buf()).into());
        }

        let config = Config::default();
        let config_str = toml::to_string(&config)?;
        fs::write(path, config_str)?;

        Ok(())
    }
}

/// Private API for the configuration.
impl Config {
    /// Returns the default for `include`.
    fn default_include() -> Vec<String> {
        vec![".dotbak/".to_string() + CONFIG_FILE_NAME]
    }

    /// Returns the default for `exclude`.
    fn default_exclude() -> Vec<String> {
        vec![".dotbak/".to_string() + REPO_FOLDER_NAME]
    }
}
