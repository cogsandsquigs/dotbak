#![cfg(test)]

use super::*;
use assert_fs::{prelude::FileTouch, NamedTempFile};

/// Test if the default configuration can be loaded from a file that doesn't exist.
#[test]
#[should_panic(expected = "ConfigNotFound")]
fn test_load_config_file_absent() {
    // This does not create a file, but just gives a (temp) path to said file.
    let config_path = NamedTempFile::new("config.toml").unwrap();

    // THIS SHOULD PANIC
    let _config = Config::load_config(&config_path).unwrap();
}

/// Tests if the default configuration can be loaded from a file that exists.
#[test]
fn test_load_config_file_exists() {
    // This does not create a file, but just gives a (temp) path to said file.
    let config_path = NamedTempFile::new("config.toml").unwrap();
    // *Now* we create the file.
    FileTouch::touch(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    let config = Config::load_config(&config_path).unwrap();

    // It should contain the default configuration.
    assert_eq!(
        config,
        Config {
            path: config_path.to_path_buf(),
            ..Config::default()
        }
    );
}

// TODO: test loading config from a file that already exists.
