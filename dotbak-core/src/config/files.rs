use crate::{CONFIG_FILE_NAME, REPO_FOLDER_NAME};
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

    /// The exclusion patterns for files to backup. This is a list of glob patterns to match
    /// against the files in the home directory. These are all relative to the home directory.
    /// When both include and exclude patterns match a file, the exclude pattern takes precedence.
    /// The default value is `[".dotbak/dotfiles"]`, which is the dotfile repository.
    #[serde(default = "FilesConfig::default_exclude")]
    pub exclude: Vec<PathBuf>,
}

impl Default for FilesConfig {
    /// The default configuration for Dotbak.
    fn default() -> Self {
        FilesConfig {
            include: FilesConfig::default_include(),
            exclude: FilesConfig::default_exclude(),
        }
    }
}

/// Private API for the configuration.
impl FilesConfig {
    /// Returns the default for `include`.
    fn default_include() -> Vec<PathBuf> {
        vec![PathBuf::from(".dotbak/").join(CONFIG_FILE_NAME)]
    }

    /// Returns the default for `exclude`.
    fn default_exclude() -> Vec<PathBuf> {
        vec![PathBuf::from(".dotbak/").join(REPO_FOLDER_NAME)]
    }
}
