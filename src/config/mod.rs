pub mod files;
mod tests;

use crate::errors::{config::ConfigError, io::IoError, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fs, path::PathBuf};

use self::files::FilesConfig;

/// The configuration that Dotbak uses to run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// The location of the configuration file. This is a temporary value that will be overwritten
    /// later when loading in `Config::load_config`, so it is not serialized.
    #[serde(skip)]
    pub path: PathBuf,

    /// The URL for the remote git repository. This is the URL that will be used to clone the
    /// repository if it doesn't exist, and to push and pull changes to and from the repository.
    /// Also, incase the local repository is deleted or corrupted, this URL will be used to clone
    /// the repository again.
    pub repository_url: Option<String>,

    /// The configuration for the `Files` struct. This is a list of files and folders that will be
    /// managed by Dotbak.
    #[serde(default)]
    pub files: FilesConfig,
}

impl Default for Config {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        Config {
            path: PathBuf::new(), // This is a temporary value that will be overwritten later.
            repository_url: None, // No default value.
            files: FilesConfig::default(),
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
            return Err(ConfigError::NotFound {
                path: path.to_path_buf(),
            }
            .into());
        }

        let config_str = fs::read_to_string(path).map_err(|err| IoError::Read {
            source: err,
            path: path.to_path_buf(),
        })?;

        config = toml::from_str(&config_str)?;

        // IMPORTANT: This is the only place where the path is set.
        config.path = path.to_path_buf();

        Ok(config)
    }

    /// Saves the config file to the given path. If the path doesn't exist, it will return an error.
    pub fn save_config(&self) -> Result<()> {
        if !self.path.exists() {
            return Err(ConfigError::NotFound {
                path: self.path.to_path_buf(),
            }
            .into());
        }

        let config_str = toml::to_string_pretty(self)?;
        fs::write(&self.path, config_str).map_err(|err| IoError::Write {
            source: err,
            path: self.path.to_path_buf(),
        })?;

        Ok(())
    }

    /// Creates a new config file at the given path. If the path already exists, it will return an error.
    pub fn create_config<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path: &Path = path.as_ref();

        if path.exists() {
            return Err(ConfigError::AlreadyExists {
                path: path.to_path_buf(),
            }
            .into());
        } else {
            // Create the file if it doesn't exist.
            fs::create_dir_all(path.parent().unwrap()).map_err(|err| IoError::Create {
                source: err,
                path: path.to_path_buf(),
            })?;

            // Create the file.
            fs::File::create(path).map_err(|err| IoError::Create {
                source: err,
                path: path.to_path_buf(),
            })?;
        }

        let mut config = Config::default();
        let config_str = toml::to_string(&config)?;

        fs::write(path, config_str).map_err(|err| IoError::Write {
            source: err,
            path: path.to_path_buf(),
        })?;

        // IMPORTANT: This is the only place where the path is set.
        config.path = path.to_path_buf();

        Ok(config)
    }

    /// Deletes the config file at the given path. If the path doesn't exist, it will return an error.
    pub fn delete_config(self) -> Result<()> {
        if !self.path.exists() {
            return Err(ConfigError::NotFound { path: self.path }.into());
        }

        fs::remove_file(&self.path).map_err(|err| IoError::Write {
            source: err,
            path: self.path,
        })?;

        Ok(())
    }
}
