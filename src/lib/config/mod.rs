mod tests;

use crate::errors::{DotbakError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};

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
    /// Loads the config file from the given path. If the path doesn't exist, it will return an error.
    pub fn load_config<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut config: Config;

        if !path.exists() {
            return Err(DotbakError::ConfigNotFound(path.to_path_buf()));
        }

        let config_str = fs::read_to_string(path)?;
        config = toml::from_str(&config_str)?;
        config.path = path.to_path_buf();

        Ok(config)
    }
}
