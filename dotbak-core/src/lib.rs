pub mod errors;

mod config;
mod files;
mod git;
mod test_util;
mod tests;

use config::Config;
use errors::{config::ConfigError, DotbakError, Result};
use files::Files;
use git::Repository;
use itertools::Itertools;
use std::path::Path;

/// The name of the configuration file.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// The name of the git repository folder.
pub(crate) const REPO_FOLDER_NAME: &str = "dotfiles";

/// The main structure to manage `dotbak`'s actions and such.
pub struct Dotbak {
    /// The configuration for `dotbak`.
    config: Config,

    /// The repository for `dotbak`.
    repo: Repository,

    /// The dotfiles that are being managed by `dotbak`.
    dotfiles: Files,
}

/// Public API for `Dotbak`.
impl Dotbak {
    /// Create a new instance of `dotbak`. If the configuration file does not exist, it will be created.
    /// If it does exist, it will be loaded.
    pub fn init() -> Result<Self> {
        let mut dotbak = Self::init_into_dirs(
            dirs::home_dir().expect("You should have a home directory!"),
            dirs::config_dir().expect("You should have a config directory!"),
            dirs::state_dir().unwrap_or_else(|| {
                dirs::data_local_dir().expect("You should have a data directory!")
            }),
        )?;

        dotbak.sync()?;

        Ok(dotbak)
    }

    /// Clone a remote repository to the local repository. If the local repository already exists, it will be
    /// deleted and re-cloned.
    pub fn clone(url: &str) -> Result<Self> {
        let mut dotbak = Self::clone_into_dirs(
            dirs::home_dir().expect("You should have a home directory!"),
            dirs::config_dir().expect("You should have a config directory!"),
            dirs::state_dir().unwrap_or_else(|| {
                dirs::data_local_dir().expect("You should have a data directory!")
            }),
            url,
        )?;

        dotbak.sync()?;

        Ok(dotbak)
    }

    /// Creates a new instance of `dotbak`. If the configuration file does not exist, an error will be returned.
    /// If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        Self::load_into_dirs(
            dirs::home_dir().expect("You should have a home directory!"),
            dirs::config_dir().expect("You should have a config directory!"),
            dirs::state_dir().unwrap_or_else(|| {
                dirs::data_local_dir().expect("You should have a data directory!")
            }),
        )
    }

    /// Sync the state. I.e., load all the files that are supposed to be loaded through `files.include`.
    pub fn sync(&mut self) -> Result<()> {
        self.dotfiles.move_and_symlink(&self.config.files.include)
    }

    /// Add a set of files/folders to the repository. This will move the files/folders to the repository and
    /// symlink them to their original location. It also writes their paths to the configuration file in the `include`
    /// list.
    pub fn add<P>(&mut self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Add the paths to the `include` list.
        self.config
            .files
            .include
            .extend(files.iter().map(|p| p.as_ref().to_path_buf()));

        self.config.save_config()?;

        // Move the files/folders to the repository and symlink them to their original location.
        self.dotfiles.move_and_symlink(files)?;

        // Commit to the repository.
        // TODO: Make this message configurable.
        self.repo.commit(&format!(
            "Add files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;

        Ok(())
    }

    /// Remove a set of files/folders from the repository. This will remove the files/folders from the repository
    /// and restore them to their original location. It also removes their paths from the configuration file in the
    /// `include` list.
    pub fn remove<P>(&mut self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Remove the paths from the `include` list.
        self.config
            .files
            .include
            .retain(|p| !files.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;

        // Remove the files/folders from the repository and restore them to their original location.
        self.dotfiles.remove_and_restore(files)?;

        // Commit to the repository.
        // TODO: Make this message configurable.
        self.repo.commit(&format!(
            "Remove files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;

        Ok(())
    }

    /// Push the repository to the remote.
    /// TODO: Logging/tracing and such.
    pub fn push(&mut self) -> Result<()> {
        self.repo.push()?;

        Ok(())
    }

    /// Pull changes from the remote.
    /// TODO: Logging/tracing and such.
    pub fn pull(&mut self) -> Result<()> {
        self.repo.pull()?;

        Ok(())
    }

    /// Run an arbitrary git command on the repository.
    pub fn arbitrary_git_command(&mut self, args: &[&str]) -> Result<()> {
        self.repo.arbitrary_command(args)?;

        Ok(())
    }

    // Deinitializes `dotbak`, removing the configuration file and the repository. This also restores all files
    // that were managed by `dotbak` to their original location.
    // TODO: Make this work.
}

/// Private API for `Dotbak`. These are mainly used for testing.
impl Dotbak {
    /// Initialize a new instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`. The user's home directory is assumed to be `<home>`.
    /// TODO: Link files/folders from the repository to the home directory based on config.
    fn init_into_dirs<P1, P2, P3>(home: P1, config: P2, repo: P3) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
    {
        let config_path = config.as_ref().to_path_buf();
        let repo_path = repo.as_ref().to_path_buf();
        let home_path = home.as_ref().to_path_buf();

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
        let repo = Repository::init(&repo_path, None)?;

        Ok(Dotbak {
            // TODO: Cloning is ugly, but it's the only concise way I can think of to get around the borrow checker
            // to allow us to have a mutable reference to `config` and `dotfiles` while also having an immutable reference
            // to `config.files` in `dotfiles`.
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`.
    /// TODO: Link files/folders from the repository to the home directory based on config.
    fn load_into_dirs<P1, P2, P3>(home: P1, config: P2, repo: P3) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
    {
        let config_path = config.as_ref().to_path_buf();
        let repo_path = repo.as_ref().to_path_buf();
        let home_path = home.as_ref().to_path_buf();

        // Load the configuration file and the repository.
        let config = Config::load_config(config_path)?;
        let repo = Repository::load(&repo_path)?;

        Ok(Dotbak {
            // TODO: Cloning is ugly, but it's the only concise way I can think of to get around the borrow checker
            // to allow us to have a mutable reference to `config` and `dotfiles` while also having an immutable reference
            // to `config.files` in `dotfiles`.
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }

    /// Clone an instance of `dotbak`, cloning the repository from the given URL to `<dotbak>/dotfiles`.
    /// The user's home directory is assumed to be `<home>`.
    /// TODO: Link files/folders from the repository to the home directory based on config.
    fn clone_into_dirs<P1, P2, P3>(home: P1, config: P2, repo: P3, url: &str) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
        P3: AsRef<Path>,
    {
        let config_path = config.as_ref().to_path_buf();
        let repo_path = repo.as_ref().to_path_buf();
        let home_path = home.as_ref().to_path_buf();

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
        let repo = Repository::clone(&repo_path, url)?;

        Ok(Dotbak {
            // TODO: Cloning is ugly, but it's the only concise way I can think of to get around the borrow checker
            // to allow us to have a mutable reference to `config` and `dotfiles` while also having an immutable reference
            // to `config.files` in `dotfiles`.
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }
}
