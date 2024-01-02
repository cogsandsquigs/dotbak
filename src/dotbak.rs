use crate::{
    config::Config,
    errors::{config::ConfigError, DotbakError, Result},
    files::Files,
    git::Repository,
    spinners::Spinner,
};
use itertools::Itertools;
use std::path::{Path, PathBuf};

/// The path to the configuration file, relative to `XDG_CONFIG_HOME`.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// The path to the git repository folder, relative to `XDG_DATA_HOME`.
pub(crate) const REPO_FOLDER_NAME: &str = "dotfiles";

/* The action messages for certain actions */
const COMMIT_MSG: &str = "📦 Committing changes";
const PUSH_MSG: &str = "📤 Pushing changes";
const PULL_MSG: &str = "📥 Pulling changes";
const SYNC_MSG: &str = "🔄 Syncing state";
const UPDATE_CONF_MSG: &str = "💾 Updating configuration";
const RM_FILES_MSG: &str = "🗑️ Removing files";
const RESTORE_FILES_MSG: &str = "⏪ Restoring files";
const RM_CONFG_MSG: &str = "🗑️ Removing configuration";
const RM_REPO_MSG: &str = "🗑️ Removing repository";

/// The main structure to manage `dotbak`'s actions and such.
pub struct Dotbak {
    /// The configuration for `dotbak`.
    pub(crate) config: Config,

    /// Whether we are verbose or not with logging. Currently does nothing.
    pub(crate) verbose: bool,

    /// The repository for `dotbak`.
    pub(crate) repo: Repository,

    /// The dotfiles that are being managed by `dotbak`.
    pub(crate) dotfiles: Files,
}

/// Public API for `Dotbak`.
impl Dotbak {
    /// Create a new instance of `dotbak`. If the configuration file does not exist, it will be created.
    /// If it does exist, it will be loaded.
    pub fn init(verbose: bool) -> Result<Self> {
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::init_into_dirs(home, config, repo, verbose)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Clone a remote repository to the local repository. If the local repository already exists, it will be
    /// deleted and re-cloned.
    pub fn clone(url: &str, verbose: bool) -> Result<Self> {
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::clone_into_dirs(home, config, repo, url, verbose)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Creates a new instance of `dotbak` from pre-defined configuration. If the configuration file does not exist,
    /// an error will be returned. If it does exist, it will be loaded.
    pub fn load(verbose: bool) -> Result<Self> {
        let (home, config, repo) = get_dotbak_dirs();
        let mut dotbak = Self::load_into_dirs(home, config, repo, verbose)?;

        dotbak.sync_all_files()?;

        Ok(dotbak)
    }

    /// Sync the state. I.e., load all the files that are supposed to be loaded through `files.include`.
    pub fn sync(&mut self) -> Result<()> {
        self.sync_all_files()?;

        // Commit to the repository.
        let spinner = spinner_progress(COMMIT_MSG, 1, 4);
        self.repo.commit("Sync files")?;
        spinner.close();

        // Pull from the repository.
        let spinner = spinner_progress(PULL_MSG, 2, 4);
        self.repo.pull()?;
        spinner.close();

        // Push to the repository.
        let spinner = spinner_progress(PUSH_MSG, 3, 4);
        self.repo.push()?;
        spinner.close();

        // Sync all files again.
        let spinner = spinner_progress(SYNC_MSG, 4, 4);
        self.sync_all_files()?;
        spinner.close();

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
        let spinner = spinner_progress(UPDATE_CONF_MSG, 1, 3);
        self.config
            .files
            .include
            .extend(files.iter().map(|p| p.as_ref().to_path_buf()));

        self.config.save_config()?;
        spinner.close();

        // Move the files/folders to the repository and symlink them to their original location.
        let spinner = spinner_progress(SYNC_MSG, 2, 3);
        self.sync_files(files)?;
        spinner.close();

        // Commit to the repository.
        // TODO: Make this message configurable.
        let spinner = spinner_progress(COMMIT_MSG, 3, 3);
        self.repo.commit(&format!(
            "Add files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;
        spinner.close();

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
        let spinner = spinner_progress(UPDATE_CONF_MSG, 1, 3);
        self.config
            .files
            .include
            .retain(|p| !files.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;
        spinner.close();

        // Remove the files/folders from the repository and restore them to their original location.
        let spinner = spinner_progress(RM_FILES_MSG, 2, 3);
        self.dotfiles.remove_and_restore(files)?;
        spinner.close();

        // Commit to the repository.
        // TODO: Make this message configurable.
        let spinner = spinner_progress(COMMIT_MSG, 3, 3);
        self.repo.commit(&format!(
            "Remove files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;
        spinner.close();

        Ok(())
    }

    /// Push the repository to the remote.
    /// TODO: Logging/tracing and such.
    pub fn push(&mut self) -> Result<()> {
        let spinner = spinner_progress(SYNC_MSG, 1, 2);
        self.sync_all_files()?;
        spinner.close();

        let spinner = spinner_progress(PUSH_MSG, 1, 2);
        self.repo.push()?;
        spinner.close();

        Ok(())
    }

    /// Pull changes from the remote.
    /// TODO: Logging/tracing and such.
    pub fn pull(&mut self) -> Result<()> {
        let spinner = spinner_progress(PULL_MSG, 1, 2);
        self.repo.pull()?;
        spinner.close();

        let spinner = spinner_progress(SYNC_MSG, 2, 2);
        self.sync_all_files()?;
        spinner.close();

        Ok(())
    }

    /// Run an arbitrary git command on the repository.
    pub fn arbitrary_git_command(&mut self, args: &[&str]) -> Result<()> {
        let spinner = spinner_progress(&format!("🏃 Running 'git {}'", args.join(" ")), 1, 2);
        self.repo.arbitrary_command(args)?;
        spinner.close();

        let spinner = spinner_progress(SYNC_MSG, 2, 2);
        self.sync_all_files()?;
        spinner.close();

        Ok(())
    }

    // Deinitializes `dotbak`, removing the configuration file and the repository. This also restores all files
    // that were managed by `dotbak` to their original location.
    pub fn deinit(self) -> Result<()> {
        // Restore all files that were managed by `dotbak` to their original location.
        let spinner = spinner_progress(RESTORE_FILES_MSG, 1, 3);
        self.dotfiles
            .remove_and_restore(&self.config.files.include)?;
        spinner.close();

        // Remove the configuration file.
        let spinner = spinner_progress(RM_CONFG_MSG, 2, 3);
        self.config.delete_config()?;
        spinner.close();

        // Remove the repository.
        let spinner = spinner_progress(RM_REPO_MSG, 3, 3);
        self.repo.delete()?;
        spinner.close();

        Ok(())
    }
}

/// Private API for `Dotbak`. These are mainly used for testing.
impl Dotbak {
    /// Initialize a new instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`. The user's home directory is assumed to be `<home>`.
    pub(crate) fn init_into_dirs<P1, P2, P3>(
        home: P1,
        config: P2,
        repo: P3,
        verbose: bool,
    ) -> Result<Self>
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
            Err(DotbakError::Config(ConfigError::NotFound { .. })) => {
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
            verbose,
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`.
    pub(crate) fn load_into_dirs<P1, P2, P3>(
        home: P1,
        config: P2,
        repo: P3,
        verbose: bool,
    ) -> Result<Self>
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
            verbose,
        })
    }

    /// Clone an instance of `dotbak`, cloning the repository from the given URL to `<dotbak>/dotfiles`.
    /// The user's home directory is assumed to be `<home>`.
    pub(crate) fn clone_into_dirs<P1, P2, P3>(
        home: P1,
        config: P2,
        repo: P3,
        url: &str,
        verbose: bool,
    ) -> Result<Self>
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
            Err(DotbakError::Config(ConfigError::NotFound { .. })) => {
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
            verbose,
        })
    }

    /// Synchronize all files that are supposed to be synchronized.
    pub(crate) fn sync_all_files(&mut self) -> Result<()> {
        let files = self.config.files.include.clone(); // TODO: Get rid of this clone!

        self.sync_files(&files)
    }

    /// Synchronize a select set of files.
    pub(crate) fn sync_files<P>(&mut self, files: &[P]) -> Result<()>
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

/// Print out a message with a [x/n] counter before it
fn spinner_progress(message: &str, current: usize, total: usize) -> Spinner {
    Spinner::new(message.to_string(), current, total)
}