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
use std::{
    path::{Path, PathBuf},
    process::Output,
};

/// The path to the configuration file, relative to `XDG_CONFIG_HOME`.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// The path to the git repository folder, relative to `XDG_DATA_HOME`.
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
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::init_into_dirs(home, config, repo)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Clone a remote repository to the local repository. If the local repository already exists, it will be
    /// deleted and re-cloned.
    pub fn clone(url: &str) -> Result<Self> {
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::clone_into_dirs(home, config, repo, url)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Creates a new instance of `dotbak` from pre-defined configuration. If the configuration file does not exist,
    /// an error will be returned. If it does exist, it will be loaded.
    pub fn load() -> Result<Self> {
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::load_into_dirs(home, config, repo)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Sync the state. I.e., load all the files that are supposed to be loaded through `files.include`.
    pub fn sync(&mut self) -> Result<()> {
        self.sync_all_files()?;

        // Commit to the repository.
        self.repo.commit("Sync files")?;

        // Pull from the repository.
        self.repo.pull()?;

        // Push to the repository.
        self.repo.push()?;

        Ok(())
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
        self.sync_files(files)?;

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
    pub fn push(&mut self) -> Result<Output> {
        self.sync_all_files()?;

        self.repo.push()
    }

    /// Pull changes from the remote.
    /// TODO: Logging/tracing and such.
    pub fn pull(&mut self) -> Result<Output> {
        let output = self.repo.pull()?;

        self.sync_all_files()?;

        Ok(output)
    }

    /// Run an arbitrary git command on the repository.
    pub fn arbitrary_git_command(&mut self, args: &[&str]) -> Result<Output> {
        let output = self.repo.arbitrary_command(args)?;

        self.sync_all_files()?;

        Ok(output)
    }

    // Deinitializes `dotbak`, removing the configuration file and the repository. This also restores all files
    // that were managed by `dotbak` to their original location.
    pub fn deinit(self) -> Result<()> {
        // Restore all files that were managed by `dotbak` to their original location.
        self.dotfiles
            .remove_and_restore(&self.config.files.include)?;

        // Remove the configuration file.
        self.config.delete_config()?;

        // Remove the repository.
        self.repo.delete()?;

        Ok(())
    }
}

/// Private API for `Dotbak`. These are mainly used for testing.
impl Dotbak {
    /// Initialize a new instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`. The user's home directory is assumed to be `<home>`.
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
            Err(DotbakError::Config(ConfigError::ConfigNotFound { .. })) => {
                Config::create_config(config_path)?
            }

            // If the error is not a `ConfigNotFound` error, return it.
            Err(err) => return Err(err),
        };

        // Try to load the repository.
        let repo = Repository::init(&repo_path, None)?;

        Ok(Dotbak {
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`.
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
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }

    /// Clone an instance of `dotbak`, cloning the repository from the given URL to `<dotbak>/dotfiles`.
    /// The user's home directory is assumed to be `<home>`.
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
            Err(DotbakError::Config(ConfigError::ConfigNotFound { .. })) => {
                Config::create_config(config_path)?
            }

            // If the error is not a `ConfigNotFound` error, return it.
            Err(err) => return Err(err),
        };

        // Try to load the repository.
        let repo = Repository::clone(&repo_path, url)?;

        Ok(Dotbak {
            dotfiles: Files::init(home_path, repo_path),
            config,
            repo,
        })
    }

    /// Synchronize all files that are supposed to be synchronized.
    fn sync_all_files(&mut self) -> Result<()> {
        let files = self.config.files.include.clone(); // TODO: Get rid of this clone!

        self.sync_files(&files)
    }

    /// Synchronize a select set of files.
    fn sync_files<P>(&mut self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Move the files/folders to the repository and symlink them to their original location.
        self.dotfiles.move_and_symlink(files)?;

        // Synchronize the files/folders.
        self.dotfiles.symlink_back_home(files)?;

        Ok(())
    }
}

/// Get the directories that `dotbak` uses. In order, it returns the `<home>`, `<config>`, and `<repo>` dirs.
fn get_dotbak_dirs() -> (PathBuf, PathBuf, PathBuf) {
    let home_dir = dirs::home_dir().expect("You should have a home directory!");
    let dotbak_dir = home_dir.join(".dotbak");

    (
        home_dir,
        dotbak_dir.join(CONFIG_FILE_NAME),
        dotbak_dir.join(REPO_FOLDER_NAME),
    )
}
