pub mod config;
pub mod error;

use config::Config;
use error::Error;
use home::home_dir;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    process::Command,
};

/// `Dotback` is sort of the "backend" for dotback. It manages dotfiles, configuration, and the
/// syncing process.
pub struct Dotback {
    /// The configuration for dotback.
    config: Config,

    /// The user's home directory.
    home_path: PathBuf,

    /// The path to the `.dotback` directory.
    dotback_path: PathBuf,
}

/// Public API for `Dotback`.
impl Dotback {
    /// Loads a `Dotback` instance from pre-existing configuration. If the configuration does not
    /// exist, it returns an error.
    /// Note that the configuration is loaded from the default location, `~/.dotback/config.toml`.
    pub fn load() -> Result<Self, Error> {
        let home_path = home_dir().unwrap(); // TODO: get rid of unwrap.
        let dotback_path = home_path.join(".dotback");

        let mut dotback = Dotback {
            config: Config::default(),
            home_path,
            dotback_path,
        };

        // If the `.dotback` directory does not exist, return an error.
        if !dotback.dotback_path.exists() {
            return Err(Error::DotbackDirectoryNotFound);
        }

        dotback.read_config()?;

        Ok(dotback)
    }

    /// Initializes a `Dotback` instance with the default configuration. If the configuration already
    /// exists, it uninstalls itself and returns an error.
    /// Note that the configuration is stored to the default location, `~/.dotback/config.toml`.
    pub fn init(repository: &str) -> Result<Dotback, Error> {
        let home_path = home_dir().unwrap(); // TODO: get rid of unwrap.
        let dotback_path = home_path.join(".dotback");

        let mut dotback = Dotback {
            config: Config::default(),
            home_path,
            dotback_path,
        };

        // If the `.dotback` directory already exists, return an error.
        if dotback.dotback_path.exists() {
            return Err(Error::DotbackDirectoryAlreadyExists);
        }

        // Create the `.dotback` directory.
        fs::create_dir_all(&dotback.dotback_path)?;

        // Init the repository.
        if let Err(e) = dotback.init_repo(repository) {
            dotback.uninstall()?;
            return Err(e);
        }

        Ok(dotback)
    }

    /// Uninstalls dotback by removing the `.dotback` directory.
    pub fn uninstall(&self) -> Result<(), Error> {
        fs::remove_dir_all(&self.dotback_path)?;

        Ok(())
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    pub fn add_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) -> Result<(), Error> {
        self.config.add_dotfile(dotfile);

        self.write_config()?;

        todo!();

        Ok(())
    }

    /// Removes a dotfile inclusion pattern to the configuration.
    pub fn remove_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) -> Result<(), Error> {
        self.config.remove_dotfile(dotfile);

        self.write_config()?;

        todo!();

        Ok(())
    }
}

/// Private API for `Dotback`.
impl Dotback {
    /// Returns the path to the configuration file.
    fn config_path(&self) -> PathBuf {
        self.dotback_path.join("config.toml")
    }

    /// Returns the path to the git repository.
    fn repository_path(&self) -> PathBuf {
        self.dotback_path.join("dotfiles")
    }

    /// Set the config to the default configuration.
    fn set_config_default(&mut self) {
        self.config = Config::default();
    }

    /// Writes the `Config` instance to a configuration file.
    /// Note that the configuration is stored to the default location, `~/.dotback/config.toml`.
    fn write_config(&self) -> Result<(), Error> {
        let content = toml::to_vec(&self.config)?;

        let mut file = File::create(self.config_path())?;

        // Write the contents of `content` to `file`.
        file.write_all(&content)?;

        Ok(())
    }

    /// Reads the configuration file and stores it in the `config` field. If the file is empty,
    /// the default configuration is used. If the file does not exist, it returns an error.
    /// Note that the configuration is read from the default location, `~/.dotback/config.toml`.
    fn read_config(&mut self) -> Result<(), Error> {
        let mut file = File::open(self.config_path())?;

        // If the file is empty, use the default configuration.
        if file.metadata()?.len() == 0 {
            self.set_config_default();
            return Ok(());
        }

        let mut contents = Vec::new();

        // Read all of the contents of `file` into `contents`.
        file.read_to_end(&mut contents)?;

        let config = toml::from_slice(&contents)?;

        self.config = config;

        Ok(())
    }

    /// Create the repository if it does not exist.
    fn init_repo(&mut self, repository: &str) -> Result<(), Error> {
        self.config.repository = repository.to_string();
        self.write_config()?;

        // Create the repository directory if it does not exist.
        if !self.repository_path().exists() {
            fs::create_dir_all(self.repository_path())?;
        }

        let output = Command::new("git")
            .arg("clone")
            .arg(repository)
            .arg(self.repository_path())
            .output()?;

        if !output.status.success() {
            Err(Error::Command {
                stderr: String::from_utf8(output.stderr).unwrap(), // TODO: get rid of unwrap.
            })
        } else {
            Ok(())
        }
    }

    /// Syncs the dotfiles with the repository.
    fn sync(&self) -> Result<(), Error> {
        todo!();
    }
}
