mod logger;
mod spinners;
mod tests;

use self::{logger::Logger, spinners::Spinner};
use crate::{
    config::Config,
    errors::{config::ConfigError, DotbakError, Result},
    files::Files,
    git::Repository,
};
use itertools::Itertools;
use std::path::{Path, PathBuf};

/// The path to the configuration file, relative to `XDG_CONFIG_HOME`.
pub(crate) const CONFIG_FILE_NAME: &str = "config.toml";

/// The path to the git repository folder, relative to `XDG_DATA_HOME`.
pub(crate) const REPO_FOLDER_NAME: &str = "dotfiles";

/// The padding used before sub-messages
const PAD: &str = "   ";

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
    config: Config,

    /// The repository for `dotbak`.
    repo: Repository,

    /// The dotfiles that are being managed by `dotbak`.
    dotfiles: Files,

    /// The logger for `dotbak`.
    logger: Logger,
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
        // Make sure everything's up to date.
        self.sync_all_files()?;

        let [mut commit_spinner, mut pull_spinner, mut push_spinner, mut sync_spinner] =
            get_spinners([COMMIT_MSG, PULL_MSG, PUSH_MSG, SYNC_MSG]);

        // Commit to the repository.
        commit_spinner.start();
        let outputs = self.repo.commit("Sync files")?;
        commit_spinner.close();
        self.logger.log_outputs(outputs);

        // Pull from the repository.
        pull_spinner.start();
        let output = self.repo.pull()?;
        pull_spinner.close();
        self.logger.log_output(output);

        // Push to the repository.
        push_spinner.start();
        let output = self.repo.push()?;
        push_spinner.close();
        self.logger.log_output(output);

        // Sync all files again.
        sync_spinner.start();
        self.sync_all_files()?;
        sync_spinner.close();
        self.logger.info(format!(
            "Synced files: {}",
            self.config
                .files
                .include
                .iter()
                .map(|f| f.display())
                .join(", ")
        ));

        Ok(())
    }

    /// Add a set of files/folders to the repository. This will move the files/folders to the repository and
    /// symlink them to their original location. It also writes their paths to the configuration file in the `include`
    /// list.
    pub fn add<P>(&mut self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let [mut update_conf_spinner, mut sync_spinner, mut commit_spinner] =
            get_spinners([UPDATE_CONF_MSG, SYNC_MSG, COMMIT_MSG]);

        // Add the paths to the `include` list.
        update_conf_spinner.start();
        self.config
            .files
            .include
            .extend(files.iter().map(|p| p.as_ref().to_path_buf()));

        self.config.save_config()?;
        update_conf_spinner.close();
        self.logger.info(format!(
            "Added files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ));

        // Move the files/folders to the repository and symlink them to their original location.
        sync_spinner.start();
        self.sync_files(files)?;
        sync_spinner.close();
        self.logger.info(format!(
            "Synced files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ));

        // Commit to the repository.
        // TODO: Make this message configurable.
        commit_spinner.start();
        let outputs = self.repo.commit(&format!(
            "Add files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;
        commit_spinner.close();
        self.logger.log_outputs(outputs);

        Ok(())
    }

    /// Remove a set of files/folders from the repository. This will remove the files/folders from the repository
    /// and restore them to their original location. It also removes their paths from the configuration file in the
    /// `include` list.
    pub fn remove<P>(&mut self, files: &[P]) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let [mut update_conf_spinner, mut rm_files_spinner, mut commit_spinner] =
            get_spinners([UPDATE_CONF_MSG, RM_FILES_MSG, COMMIT_MSG]);

        // Remove the paths from the `include` list.
        update_conf_spinner.start();
        self.config
            .files
            .include
            .retain(|p| !files.iter().any(|p2| p == p2.as_ref()));

        // Save the configuration file.
        self.config.save_config()?;
        update_conf_spinner.close();
        self.logger.info(format!(
            "Removed files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ));

        // Remove the files/folders from the repository and restore them to their original location.
        rm_files_spinner.start();
        self.dotfiles.remove_and_restore(files)?;
        rm_files_spinner.close();
        self.logger.info(format!(
            "Restored files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ));

        // Commit to the repository.
        // TODO: Make this message configurable.
        commit_spinner.start();
        let outputs = self.repo.commit(&format!(
            "Remove files: {}",
            files.iter().map(|p| p.as_ref().display()).join(", ")
        ))?;
        commit_spinner.close();
        self.logger.log_outputs(outputs);

        Ok(())
    }

    /// Push the repository to the remote.
    /// TODO: Logging/tracing and such.
    pub fn push(&mut self) -> Result<()> {
        let [mut sync_spinner, mut push_spinner] = get_spinners([SYNC_MSG, PUSH_MSG]);

        sync_spinner.start();
        self.sync_all_files()?;
        sync_spinner.close();
        self.logger.info(format!(
            "Synced files: {}",
            self.config
                .files
                .include
                .iter()
                .map(|f| f.display())
                .join(", ")
        ));

        push_spinner.start();
        let output = self.repo.push()?;
        push_spinner.close();
        self.logger.log_output(output);

        Ok(())
    }

    /// Pull changes from the remote.
    /// TODO: Logging/tracing and such.
    pub fn pull(&mut self) -> Result<()> {
        let [mut pull_spinner, mut sync_spinner] = get_spinners([PULL_MSG, SYNC_MSG]);

        pull_spinner.start();
        let output = self.repo.pull()?;
        pull_spinner.close();
        self.logger.log_output(output);

        sync_spinner.start();
        self.sync_all_files()?;
        sync_spinner.close();
        self.logger.info(format!(
            "Synced files: {}",
            self.config
                .files
                .include
                .iter()
                .map(|f| f.display())
                .join(", ")
        ));

        Ok(())
    }

    /// Run an arbitrary git command on the repository.
    pub fn arbitrary_git_command(&mut self, args: &[&str]) -> Result<()> {
        let [mut arbitrary_command_spinner, mut sync_spinner] =
            get_spinners(["🏃 Running arbitrary git command", SYNC_MSG]);

        arbitrary_command_spinner.start();
        let output = self.repo.arbitrary_command(args)?;
        arbitrary_command_spinner.close();
        self.logger.log_output(output);

        sync_spinner.start();
        self.sync_all_files()?;
        sync_spinner.close();
        self.logger.info(format!(
            "Synced files: {}",
            self.config
                .files
                .include
                .iter()
                .map(|f| f.display())
                .join(", ")
        ));

        Ok(())
    }

    // Deinitializes `dotbak`, removing the configuration file and the repository. This also restores all files
    // that were managed by `dotbak` to their original location.
    pub fn deinit(self) -> Result<()> {
        let [mut restore_files_spinner, mut rm_config_spinner, mut rm_repo_spinner] =
            get_spinners([RESTORE_FILES_MSG, RM_CONFG_MSG, RM_REPO_MSG]);

        // Restore all files that were managed by `dotbak` to their original location.
        restore_files_spinner.start();
        self.dotfiles
            .remove_and_restore(&self.config.files.include)?;
        restore_files_spinner.close();
        self.logger.info(format!(
            "Restored files: {}",
            self.config
                .files
                .include
                .iter()
                .map(|f| f.display())
                .join(", ")
        ));

        // Remove the configuration file.
        rm_config_spinner.start();
        self.config.delete_config()?;
        rm_config_spinner.close();

        // Remove the repository.
        rm_repo_spinner.start();
        self.repo.delete()?;
        rm_repo_spinner.close();

        Ok(())
    }
}

/// Private API for `Dotbak`. These are mainly used for testing.
impl Dotbak {
    /// Initialize a new instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`. The user's home directory is assumed to be `<home>`.
    fn init_into_dirs<P1, P2, P3>(home: P1, config: P2, repo: P3, verbose: bool) -> Result<Self>
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
            logger: Logger::new(verbose),
        })
    }

    /// Clone an instance of `dotbak`, cloning the repository from the given URL to `<dotbak>/dotfiles`.
    /// The user's home directory is assumed to be `<home>`.
    fn clone_into_dirs<P1, P2, P3>(
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
            logger: Logger::new(verbose),
        })
    }

    /// Load an instance of `dotbak`, loading the configuration file from `<dotbak>/config.toml` and the
    /// repository from `<dotbak>/dotfiles`.
    fn load_into_dirs<P1, P2, P3>(home: P1, config: P2, repo: P3, verbose: bool) -> Result<Self>
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

            // TODO: Make this output to log file when in daemon mode.
            logger: Logger::new_with_streams(
                verbose,
                Box::new(std::io::stdout()),
                Box::new(std::io::stderr()),
            ),
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

/// Gets the spinners for a set of messages.
fn get_spinners<const N: usize>(messages: [&str; N]) -> [Spinner; N] {
    let mut spinners = vec![];

    let max_chars = messages
        .iter()
        .map(|m| m.len())
        .max()
        .expect("This should not fail!");

    for (i, message) in messages.iter().enumerate() {
        spinners.push(Spinner::new(
            message.to_string(),
            PAD,
            max_chars - message.len(),
            i + 1,
            N,
        ));
    }

    spinners
        .try_into()
        .expect("The number of spinners should be equal to the number of messages!")
}
