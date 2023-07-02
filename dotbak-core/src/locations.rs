use super::errors::Result;
use crate::errors::DotbakError;
use std::path::PathBuf;

/// The name of the configuration file.
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// The name of the git repository folder.
pub const REPO_FOLDER_NAME: &str = "dotfiles";

/// The location of the dotbak directory where configuration and backups are stored.
pub(crate) fn dotbak_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir()
        .ok_or(DotbakError::NoHomeDir)?
        .join(".dotbak");

    Ok(dir)
}
