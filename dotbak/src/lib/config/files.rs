use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The configuration for the `Files` struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilesConfig {
    /// The inclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/config.toml"]`, which is the configuration file itself.
    #[serde(default = "FilesConfig::default_include")]
    pub include: Vec<PathBuf>,
}

impl Default for FilesConfig {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        FilesConfig {
            include: FilesConfig::default_include(),
        }
    }
}

/// Private API for the configuration.
impl FilesConfig {
    /// Returns the default for `include`.
    fn default_include() -> Vec<PathBuf> {
        vec![".dotbak/config.toml".into()]
    }
}
