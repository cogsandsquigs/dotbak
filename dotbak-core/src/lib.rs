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
use std::path::{Path, PathBuf};

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
        Self::init_into_dirs(home_dir()?.join(".dotbak"), home_dir()?)
    }

    /// Creates a new instance of `dotbak`. If the configuration file does not exist, an error will be returned.
    /// If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        Self::load_into_dirs(home_dir()?.join(".dotbak"), home_dir()?)
    }

    /// Clone a remote repository to the local repository. If the local repository already exists, it will be
    /// deleted and re-cloned.
    pub fn clone(url: &str) -> Result<Self> {
        Self::clone_into_dirs(home_dir()?.join(".dotbak"), home_dir()?, url)
    }

    /// Add a set of files/folders to the repository. This will move the files/folders to the repository and
    /// symlink them to their original location. It also writes their paths to the configuration file in the `include`
    /// list, and removes them from the `exclude` list.
    /// TODO: Make this respect the `include` and `exclude` configuration options.
    pub fn add<P>(&mut self, paths: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Add the paths to the `include` list.
        self.config
            .files
            .include
            .extend(paths.iter().map(|p| p.as_ref().to_path_buf()));

        // Remove the paths from the `exclude` list.
        self.config
            .files
            .exclude
            .retain(|p| !paths.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;

        // Move the files/folders to the repository and symlink them to their original location.
        self.dotfiles.move_and_symlink(paths)?;

        // Commit to the repository.
        // TODO: Make this message configurable.
        self.repo.commit(&format!(
            "Add files: {}",
            paths.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;

        Ok(())
    }

    /// Remove a set of files/folders from the repository. This will remove the files/folders from the repository
    /// and restore them to their original location. It also removes their paths from the configuration file in the
    /// `include` list.
    /// TODO: Make this respect the `include` and `exclude` configuration options.
    pub fn remove<P>(&mut self, paths: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Remove the paths from the `include` list.
        self.config
            .files
            .include
            .retain(|p| !paths.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;

        // Remove the files/folders from the repository and restore them to their original location.
        self.dotfiles.remove_and_restore(paths)?;

        // Commit to the repository.
        // TODO: Make this message configurable.
        self.repo.commit(&format!(
            "Remove files: {}",
            paths.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;

        Ok(())
    }

    /// Excludes a set of files/folders from the repository. This will remove the files/folders from the repository
    /// and restore them to their original location. It also removes their paths from the configuration file in the
    /// `include` list, and adds them to the `exclude` list.
    pub fn exclude<P>(&mut self, paths: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Add the paths to the `exclude` list.
        self.config
            .files
            .exclude
            .extend(paths.iter().map(|p| p.as_ref().to_path_buf()));

        // Remove the paths from the `include` list.
        self.config
            .files
            .include
            .retain(|p| !paths.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;

        // Remove the files/folders from the repository and restore them to their original location.
        self.dotfiles.remove_and_restore(paths)?;

        // Commit to the repository.
        // TODO: Make this message configurable.
        self.repo.commit(&format!(
            "Remove files: {}",
            paths.iter().map(|p| p.as_ref().display()).join(", ")
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
    fn init_into_dirs<P1, P2>(home: P1, dotbak: P2) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let dotbak_dir = dotbak.as_ref();

        let config_path = dotbak_dir.join(CONFIG_FILE_NAME);
        let repo_path = dotbak_dir.join(REPO_FOLDER_NAME);
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
            config,
            repo,
            dotfiles: Files::init(home_path, repo_path),
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`.
    fn load_into_dirs<P1, P2>(home: P1, dotbak: P2) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let dotbak_dir = dotbak.as_ref();

        let config_path = dotbak_dir.join(CONFIG_FILE_NAME);
        let repo_path = dotbak_dir.join(REPO_FOLDER_NAME);

        // Load the configuration file and the repository.
        let config = Config::load_config(config_path)?;
        let repo = Repository::load(&repo_path)?;
        let home_dir = home.as_ref().to_path_buf();

        Ok(Dotbak {
            config,
            repo,
            dotfiles: Files::init(home_dir, repo_path),
        })
    }

    /// Clone an instance of `dotbak`, cloning the repository from the given URL to `<dotbak>/dotfiles`.
    /// The user's home directory is assumed to be `<home>`.
    fn clone_into_dirs<P1, P2>(home: P1, dotbak: P2, url: &str) -> Result<Self>
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        let dotbak_dir = dotbak.as_ref();

        let config_path = dotbak_dir.join(CONFIG_FILE_NAME);
        let repo_path = dotbak_dir.join(REPO_FOLDER_NAME);

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
        let home_dir = home.as_ref().to_path_buf();

        Ok(Dotbak {
            config,
            repo,
            dotfiles: Files::init(home_dir, repo_path),
        })
    }
}

/// Get the home directory for the user running `dotbak`.
fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or(DotbakError::NoHomeDir)
}
