use core::str;

use glob::Pattern;
use home::home_dir;

/// The main configuration for dotback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// The git repository where the dotfiles are synced to.
    pub repository: String,

    /// The dotfiles that we want to sync. These are absolute paths to the
    /// dotfiles.
    pub dotfiles: Vec<Pattern>,
}

/// Public API for `Config`.
impl Config {
    /// Create a new configuration with the given repository. The default dotfiles
    /// is just the `.dotback` directory.
    pub fn new<S: ToString>(repository: S) -> Self {
        let mut config = Self {
            repository: repository.to_string(),
            dotfiles: vec![],
        };

        config.add_dotfile(".dotback/*");

        config
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    /// TODO: Get rid of `unwrap`s.
    pub fn add_dotfile(&mut self, pattern: &str) {
        self.dotfiles
            .push(Pattern::new(home_dir().unwrap().join(pattern).to_str().unwrap()).unwrap());
    }
}
