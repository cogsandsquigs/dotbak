use serde::{Deserialize, Serialize};

/// The configuration that Dotback uses to run.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Config {}

impl Default for Config {
    /// The default configuration for Dotback.
    fn default() -> Self {
        Config {}
    }
}
