#![cfg(test)]

use super::*;
use assert_fs::prelude::*;

/// Test if the default configuration can be loaded from a file that doesn't exist.
#[test]
fn test_load_config_file_absent() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_path = temp.path().join("some_dir");
    let config = Config::load_config_path(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // It should contain the default configuration.
    assert_eq!(
        config,
        Config {
            path: config_path.join(".dotbak/config.toml"),
            ..Config::default()
        }
    );
}

/// test if the configuration can create sub-folders (like a new folder in ~/.config).
#[test]
fn test_load_config_file_subfolder() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_path = temp.path().join("subfolder/some_dir");
    let config = Config::load_config_path(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // It should contain the default configuration.
    assert_eq!(
        config,
        Config {
            path: config_path.join(".dotbak/config.toml"),
            ..Config::default()
        }
    );
}

// TODO: test loading config from a file that already exists.
