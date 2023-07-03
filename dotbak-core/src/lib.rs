pub mod config;
pub mod errors;
pub mod git;
pub(crate) mod test_util;
mod tests;

use config::Config;
use errors::{config::ConfigError, DotbakError, Result};
use git::GitRepo;
use std::path::PathBuf;

/// The name of the configuration file.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// The name of the git repository folder.
pub(crate) const REPO_FOLDER_NAME: &str = "dotfiles";

/// The main structure to manage `dotbak`'s actions and such.
pub struct Dotbak {
    /// The home directory for the user running `dotbak`.
    home_dir: PathBuf,

    /// The configuration for `dotbak`.
    config: Config,

    /// The repository for `dotbak`.
    repo: GitRepo,
}

/// Public API for `Dotbak`.
impl Dotbak {
    /// Create a new instance of `dotbak`. If the configuration file does not exist, it will be created.
    /// If it does exist, it will be loaded.
    pub fn init() -> Result<Self> {
        Self::init_from_dir(home_dir()?.join(".dotbak"))
    }

    /// Creates a new instance of `dotbak`. If the configuration file does not exist, an error will be returned.
    /// If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        Self::load_from_dir(home_dir()?.join(".dotbak"))
    }
}

/// Private API for `Dotbak`.
impl Dotbak {
    /// Initialize a new instance of `dotbak`, loading the configuration file from `<dir>/config.toml` and the
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
        let repo = GitRepo::init(repo_path, None)?;

        Ok(Dotbak {
            home_dir: home_dir()?,
            config,
            repo,
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dir>/config.toml` and the
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
        let repo = GitRepo::load(repo_path)?;

        Ok(Dotbak {
            home_dir: home_dir()?,
            config,
            repo,
        })
    }
}

/// Get the home directory for the user running `dotbak`.
fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or(DotbakError::NoHomeDir)
}
