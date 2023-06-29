use lazy_static::lazy_static;
use std::path::{Path, PathBuf};

lazy_static! {
    /// The location of the configuration file.
    pub static ref CONFIG_PATH: PathBuf = DOTBAK_DIR.join("config.toml");

    /// The location of the git repository where the backups are stored.
    pub static ref REPO_PATH: PathBuf = DOTBAK_DIR.join("repo");

    /// The directory of the `.dotbak` directory.
    pub static ref DOTBAK_DIR: PathBuf = dirs::home_dir()
        .expect("No home directory found for this computer! This should never happen!")
        .join(".dotbak");
}

/// Imply `AsRef<Path>` for `CONFIG_PATH`.
impl AsRef<Path> for CONFIG_PATH {
    fn as_ref(&self) -> &Path {
        self
    }
}

/// Imply `AsRef<Path>` for `REPO_PATH`.
impl AsRef<Path> for REPO_PATH {
    fn as_ref(&self) -> &Path {
        self
    }
}

/// Imply `AsRef<Path>` for `DOTBAK_DIR`.
impl AsRef<Path> for DOTBAK_DIR {
    fn as_ref(&self) -> &Path {
        self
    }
}
