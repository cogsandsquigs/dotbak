mod tests;

use crate::errors::{
    config::ConfigError,
    io::{CreateSnafu, ReadSnafu, WriteSnafu},
    Result,
};
use crate::locations::{CONFIG_FILE_NAME, REPO_FOLDER_NAME};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::path::Path;
use std::{fs, path::PathBuf};

/// The configuration that Dotbak uses to run.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/config.toml"]`, which is the configuration file itself.
    #[serde(default = "Config::default_include")]
    pub include: Vec<PathBuf>,

    /// The exclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/repo"]`, which is the configuration file itself.
    #[serde(default = "Config::default_exclude")]
    pub exclude: Vec<PathBuf>,
}

impl Default for Config {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        Config {
            path: PathBuf::new(), // This is a temporary value that will be overwritten later.
            repository_url: None, // No default value.
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
            return Err(ConfigError::ConfigNotFound {
                path: path.to_path_buf(),
            }
            .into());
        }

        let config_str = fs::read_to_string(path).context(ReadSnafu {
            path: path.to_path_buf(),
        })?;

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
            return Err(ConfigError::ConfigNotFound {
                path: path.to_path_buf(),
            }
            .into());
        }

        let config_str = toml::to_string(self)?;
        fs::write(path, config_str).context(WriteSnafu {
            path: path.to_path_buf(),
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
            return Err(ConfigError::ConfigAlreadyExists {
                path: path.to_path_buf(),
            }
            .into());
        } else {
            // Create the file if it doesn't exist.
            fs::create_dir_all(path.parent().unwrap()).context(CreateSnafu {
                path: path.to_path_buf(),
            })?;

            // Create the file.
            fs::File::create(path).context(CreateSnafu {
                path: path.to_path_buf(),
            })?;
        }

        let mut config = Config::default();
        let config_str = toml::to_string(&config)?;

        fs::write(path, config_str).context(WriteSnafu {
            path: path.to_path_buf(),
        })?;

        // IMPORTANT: This is the only place where the path is set.
        config.path = path.to_path_buf();

        Ok(config)
    }
}

/// Private API for the configuration.
impl Config {
    /// Returns the default for `include`.
    fn default_include() -> Vec<PathBuf> {
        vec![PathBuf::from(".dotbak/").join(CONFIG_FILE_NAME)]
    }

    /// Returns the default for `exclude`.
    fn default_exclude() -> Vec<PathBuf> {
        vec![PathBuf::from(".dotbak/").join(REPO_FOLDER_NAME)]
    }
}
