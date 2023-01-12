/// The main configuration for dotback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// The git repository where the dotfiles are synced to.
    pub repository: String,

    /// The dotfiles that we want to sync. These are absolute paths to the
    /// dotfiles.
    pub dotfiles: Vec<String>,

    /// Any dotfiles that we want to exclude.
    pub exclude: Vec<String>,
}

/// Public API for `Config`.
impl Config {
    /// Create a new configuration with the given repository. The default dotfiles
    /// is just the `.dotback` directory.
    pub fn new<S: ToString>(repository: S) -> Self {
        let mut config = Self {
            repository: repository.to_string(),
            dotfiles: vec![],
            exclude: vec![],
        };

        config.add_dotfile(".dotback/*");

        config
    }

    /// Adds a new dotfile inclusion pattern to the configuration.
    pub fn add_dotfile<S: ToString>(&mut self, pattern: S) {
        self.dotfiles.push(pattern.to_string());
    }

    /// Removes a dotfile inclusion pattern to the configuration.
    pub fn remove_dotfile<S: ToString>(&mut self, pattern: S) {
        todo!()
    }
}
