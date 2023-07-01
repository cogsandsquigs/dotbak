use crate::errors::DotbakError;

use super::errors::Result;
use std::path::PathBuf;

/// The name of the configuration file.
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// The name of the git repository folder.
pub const REPO_FOLDER_NAME: &str = "repo";

// lazy_static! {
//     /// The location of the configuration file.
//     pub static ref CONFIG_PATH: PathBuf = DOTBAK_DIR.join("config.toml");

//     /// The location of the git repository where the backups are stored.
//     pub static ref REPO_PATH: PathBuf = DOTBAK_DIR.join("repo");

//     /// The directory of the `.dotbak` directory.
//     pub static ref DOTBAK_DIR: PathBuf = dirs::home_dir()
//         .expect("No home directory found for this computer! This should never happen!")
//         .join(".dotbak");
// }

// Convert the above lazy_static! to a set of functions which return the same values wrapped in a Result.

/// The location of the configuration file.
pub fn config_path() -> Result<PathBuf> {
    let path = dotbak_dir()?.join(CONFIG_FILE_NAME);

    Ok(path)
}

/// The location of the git repository where the backups are stored.
pub fn repo_path() -> Result<PathBuf> {
    let path = dotbak_dir()?.join(REPO_FOLDER_NAME);

    Ok(path)
}

fn dotbak_dir() -> Result<PathBuf> {
    let dir = dirs::home_dir()
        .ok_or(DotbakError::NoHomeDir)?
        .join(".dotbak");

    Ok(dir)
}
