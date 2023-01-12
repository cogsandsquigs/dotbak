use super::dotfile::Dotfile;
use serde::{Deserialize, Serialize};

/// The main configuration for dotback.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// The git repository where the dotfiles are synced to.
    pub repository: String,

    /// The dotfiles that we want to sync. These are absolute paths to the
    /// dotfiles.
    pub dotfiles: Vec<Dotfile>,
}

/// Public API for `Config`.
impl Config {
    /// Create a new configuration with the given repository. The default set of dotfiles included
    /// is just the `.dotback` directory, as well as the `.config/*` directory (only files, not
    /// subdirectories).
    pub fn new<S: ToString>(repository: S) -> Self {
        let mut config = Self {
            repository: repository.to_string(),
            dotfiles: vec![],
        };

        config
            .add_dotfile(".dotback/*")
          //  .expect("This should be a valid glob pattern")
          ;
        config
            .add_dotfile(".config/*")
          //  .expect("This should be a valid glob pattern")
          ;

        config
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    pub fn add_dotfile<S: ToString>(&mut self, dotfile: S) {
        self.dotfiles.push(Dotfile::new(dotfile))
    }

    /// Removes a dotfile inclusion pattern to the configuration.
    pub fn remove_dotfile<S: ToString>(&mut self, dotfile: S) {
        todo!()
    }
}
