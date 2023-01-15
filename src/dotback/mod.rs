pub mod config;
pub mod error;

use config::Config;
use error::Error;
use home::home_dir;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

/// `Dotback` is sort of the "backend" for dotback. It manages dotfiles, configuration, and the
/// syncing process.
pub struct Dotback {
    /// The user's home directory.
    home_path: PathBuf,

    /// The path to the `.dotback` directory.
    dotback_path: PathBuf,

    /// The configuration for dotback.
    config: Config,
}

/// Public API for `Dotback`.
impl Dotback {
    /// Loads a `Dotback` instance from pre-existing configuration. If the configuration does not
    /// exist, it returns an error.
    /// Note that the configuration is loaded from the default location, `~/.dotback/config.toml`.
    pub fn load() -> Result<Self, Error> {
        let mut dotback = Dotback::new();

        // If the `.dotback` directory does not exist, return an error.
        if !dotback.dotback_path.exists() {
            return Err(Error::DotbackDirectoryNotFound);
        }

        dotback.read_config()?;

        Ok(dotback)
    }

    /// Initializes a `Dotback` instance with the default configuration.
    /// Note that the configuration is stored to the default location, `~/.dotback/config.toml`.
    pub fn init() -> Result<Dotback, Error> {
        let dotback = Dotback::new();

        fs::create_dir_all(&dotback.dotback_path)?;

        dotback.write_config()?;

        dotback.init_repository()?;

        Ok(dotback)
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    pub fn add_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) -> Result<(), Error> {
        self.config.add_dotfile(dotfile);

        self.write_config()?;

        Ok(())
    }

    /// Removes a dotfile inclusion pattern to the configuration.
    pub fn remove_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) {
        self.config.remove_dotfile(dotfile)
    }
}

/// Private API for `Dotback`.
impl Dotback {
    /// Creates a new `Dotback` instance.
    fn new() -> Self {
        let home_path = home_dir().unwrap(); // TODO: get rid of unwrap.
        let dotback_path = home_path.join(".dotback");

        Dotback {
            home_path,
            dotback_path,
            config: Config::default(),
        }
    }

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

    /// Initializes the dotback repository.
    fn init_repository(&self) -> Result<(), Error> {
        fs::create_dir_all(self.repository_path())?;

        todo!()
    }
}
