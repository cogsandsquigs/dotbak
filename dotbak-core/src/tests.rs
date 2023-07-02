#![cfg(test)]

use super::*;
use assert_fs::TempDir;

/// Test if we can initialize a new `Dotbak` instance from a directory.
#[test]
fn test_init_repo() {
    let dir = TempDir::new().unwrap();
    let dotbak_dir = dir.path();
    let result = Dotbak::init_from_dir(dotbak_dir);

    assert!(result.is_ok());
    assert!(dotbak_dir.join(CONFIG_FILE_NAME).exists());
    repo_exists!(dotbak_dir.join(REPO_FOLDER_NAME));
}

/// Test if we can initialize a new `Dotbak` instance from a directory that does not exist.
#[test]
fn test_init_repo_no_dir() {
    let dir = TempDir::new().unwrap();
    let dotbak_dir = dir.path().join("some/sub/directory");
    let result = Dotbak::init_from_dir(&dotbak_dir);

    assert!(result.is_ok());
    assert!(dotbak_dir.join(CONFIG_FILE_NAME).exists());
    repo_exists!(dotbak_dir.join(REPO_FOLDER_NAME));
}

/// Test if we can load an existing `Dotbak` instance from a directory with a configuration file
/// and a repository.
#[test]
fn test_load_repo() {
    let dir = TempDir::new().unwrap();
    let dotbak_dir = dir.path();
    let result = Dotbak::init_from_dir(dotbak_dir);

    assert!(result.is_ok());
    assert!(dotbak_dir.join(CONFIG_FILE_NAME).exists());
    repo_exists!(dotbak_dir.join(REPO_FOLDER_NAME));

    let result = Dotbak::load_from_dir(dotbak_dir);

    assert!(result.is_ok());
    assert!(dotbak_dir.join(CONFIG_FILE_NAME).exists());
    repo_exists!(dotbak_dir.join(REPO_FOLDER_NAME));
}

/// Test if we can load an existing `Dotbak` instance from a directory that has not yet been initialized.
#[test]
fn test_load_repo_no_dir() {
    let dir = TempDir::new().unwrap();
    let dotbak_dir = dir.path().join("some/sub/directory");
    let result = Dotbak::load_from_dir(dotbak_dir);

    assert!(result.is_err());
    assert!(matches!(
        result.err().unwrap(),
        DotbakError::Config {
            source: ConfigError::ConfigNotFound { .. },
        }
    ));
}
