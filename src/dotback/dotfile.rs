use serde::{Deserialize, Serialize};

/// A dotfile.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Dotfile {
    /// The path to the dotfile in question, relative with respect to the user's home directory.
    pub path: String,
}

/// Public API for `Dotfile`.
impl Dotfile {
    /// Creates a new `Dotfile`.
    pub fn new<S: ToString>(dotfile: S) -> Self {
        Self {
            path: dotfile.to_string(),
        }
    }
}
