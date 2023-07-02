pub mod config;
pub mod errors;
pub mod git;
pub(crate) mod locations;
mod tests;

#[cfg(test)]
pub(crate) mod test_util;

use config::Config;
use errors::{config::ConfigError, DotbakError, Result};
use git2::Repository;
use locations::{CONFIG_FILE_NAME, REPO_FOLDER_NAME};

/// The main structure to manage the program's actions and such.
pub struct Dotbak {
    /// The configuration for the program.
    config: Config,

    /// The repository for the program.
    repo: Repository,
}

/// Public API for the program.
impl Dotbak {
    /// Create a new instance of the program. If the configuration file does not exist, it will be created.
    /// If it does exist, it will be loaded.
    pub fn init() -> Result<Self> {
        Self::init_from_dir(locations::dotbak_dir()?)
    }

    /// Creates a new instance of the program. If the configuration file does not exist, an error will be returned.
    /// If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        Self::load_from_dir(locations::dotbak_dir()?)
    }
}

/// Private API for Dotbak.
impl Dotbak {
    /// Initialize a new instance of the program, loading the configuration file from `<dir>/config.toml` and the
    /// repository from `<dir>/dotfiles`.
    fn init_from_dir<P>(dir: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let dir = dir.as_ref();

        let config_path = dir.join(CONFIG_FILE_NAME);
        let repo_path = dir.join(REPO_FOLDER_NAME);

        // Try to load the configuration file.
        let config = match Config::load_config(&config_path) {
            // If the configuration file exists, load it.
            // TODO: log that the configuration file was loaded, not created.
            Ok(config) => config,

            // If the configuration file does not exist, create it.
            // TODO: log that the configuration file was created, not loaded.
            Err(DotbakError::Config {
                source: ConfigError::ConfigNotFound { .. },
            }) => Config::create_config(config_path)?,

            // If the error is not a `ConfigNotFound` error, return it.
            Err(err) => return Err(err),
        };

        // Try to load the repository.
        let repo = Self::init_repo(repo_path)?;

        Ok(Dotbak { config, repo })
    }

    /// Load an instance of the program, loading the configuration file from `<dir>/config.toml` and the
    /// repository from `<dir>/dotfiles`.
    fn load_from_dir<P>(dir: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        let dir = dir.as_ref();

        let config_path = dir.join(CONFIG_FILE_NAME);
        let repo_path = dir.join(REPO_FOLDER_NAME);

        // Load the configuration file and the repository.
        let config = Config::load_config(config_path)?;
        let repo = Self::load_repo(repo_path)?;

        Ok(Dotbak { config, repo })
    }
}
