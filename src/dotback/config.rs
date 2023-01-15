use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The main configuration for dotback.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// The git repository where the dotfiles are synced to.
    pub repository: String,

    /// The dotfiles that we want to sync. These are absolute paths to the
    /// dotfiles.
    pub dotfiles: Vec<PathBuf>,
}

/// Public API for `Config`.
impl Config {
    /// Create a new configuration with the given repository.
    pub fn new<S: ToString>(repository: S) -> Self {
        Self {
            repository: repository.to_string(),
            dotfiles: vec![],
        }
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    pub fn add_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) {
        self.dotfiles.push(dotfile.into())
    }

    /// Removes a dotfile inclusion pattern to the configuration.
    pub fn remove_dotfile<P: Into<PathBuf>>(&mut self, dotfile: P) {
        // Have to do this so we can take a reference to it
        let dotfile = dotfile.into();

        self.dotfiles.retain(|d| d != &dotfile)
    }
}

/// Default implementation for `Config`.
impl Default for Config {
    fn default() -> Self {
        Self {
            repository: String::new(),
            dotfiles: vec![
                PathBuf::from(".bashrc"),
                PathBuf::from(".bash_profile"),
                PathBuf::from(".zshrc"),
                PathBuf::from(".zprofile"),
            ],
        }
    }
}
