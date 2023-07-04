#![cfg(test)]

use super::*;
use assert_fs::{prelude::FileTouch, NamedTempFile, TempDir};

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

// Test if we can save the default configuration to a file that doesn't exist.
#[test]
#[should_panic(expected = "ConfigNotFound")]
fn test_save_config_file_absent() {
    // This does not create a file, but just gives a (temp) path to said file.
    let config_path = NamedTempFile::new("config.toml").unwrap();

    let config = Config {
        path: config_path.to_path_buf(),
        ..Default::default()
    };

    // THIS SHOULD PANIC
    config.save_config().unwrap();
}

// Test if we can save the default configuration to a file that exists.
#[test]
fn test_save_config_file_exists() {
    // This does not create a file, but just gives a (temp) path to said file.
    let config_path = NamedTempFile::new("config.toml").unwrap();
    // *Now* we create the file.
    FileTouch::touch(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // Create a new config file at the given path. If the path already exists, it will return an error.
    let config = Config {
        path: config_path.to_path_buf(),

        files: FilesConfig {
            // The include and exclude fields are here to make sure we are not
            // loading an empty file down the line.
            include: vec!["test1".into(), "test2".into()],
            exclude: vec!["test3".into(), "test4".into()],
        },
        ..Default::default()
    };

    config.save_config().unwrap();

    // The config file should still exist.
    assert!(config_path.exists());

    // The config file should contain the default configuration.
    let config_str = fs::read_to_string(&config_path).unwrap();
    let config_toml: Config = Config {
        path: config_path.to_path_buf(),
        ..toml::from_str(&config_str).unwrap()
    };

    assert_eq!(config_toml, config);
}

/// Tests the creation of a config file at a path that doesn't exist.
#[test]
fn test_create_config_file_absent() {
    // This does not create a file, but just gives a (temp) path to said file.
    let temp_path = TempDir::new().unwrap();
    let config_path = temp_path.path().join("some/sub/dirs/config.toml");

    // The config file should not exist.
    assert!(!config_path.exists());

    // Create a new config file at the given path. If the path already exists, it will return an error.
    Config::create_config(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // The config file should contain the default configuration.
    let config_str = fs::read_to_string(&config_path).unwrap();
    let config_toml: Config = Config {
        path: config_path.clone(),
        ..toml::from_str(&config_str).unwrap()
    };

    assert_eq!(
        config_toml,
        Config {
            path: config_path,
            ..Config::default()
        }
    );
}

/// Tests the creation of a config file at a path that already exists.
/// This should panic.
#[test]
#[should_panic(expected = "ConfigAlreadyExists")]
fn test_create_config_file_exists() {
    // This does not create a file, but just gives a (temp) path to said file.
    let config_path = NamedTempFile::new("config.toml").unwrap();
    // *Now* we create the file.
    FileTouch::touch(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // THIS SHOULD PANIC
    Config::create_config(&config_path).unwrap();
}

// TODO: test loading config from a file that already exists.
