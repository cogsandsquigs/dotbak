use serde::{Deserialize, Serialize};

/// Configuration for `Dotback`, according to the configuration file
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Configuration {
    /// The repository for where all the dotfiles are stored.
    pub repository: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            repository: "".into(),
        }
    }
}
