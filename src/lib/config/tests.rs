#![cfg(test)]

use super::*;
use assert_fs::prelude::*;

/// Test if the default configuration can be loaded from a file that doesn't exist.
#[test]
fn test_load_config_file_absent() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_path = temp.path().join("dotbak.toml");
    let config = Config::load_config_path(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // It should contain the default configuration.
    assert_eq!(config, Config::default());
}

/// Test if the default configuration can be loaded from a file that exists.
#[test]
fn test_load_config_file_present() {
    let config_path = assert_fs::NamedTempFile::new("dotbak.toml").unwrap();
    config_path.write_str("include = [\"test\"]").unwrap();

    // Create the config file.

    let config = Config::load_config_path(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // It should contain "test" in the include list.
    assert_eq!(config.include, vec!["test"]);
}

/// test if the configuration can create sub-folders (like a new folder in ~/.config).
#[test]
fn test_load_config_file_subfolder() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_path = temp.path().join("subfolder/dotbak.toml");
    let config = Config::load_config_path(&config_path).unwrap();

    // The config file should have been created.
    assert!(config_path.exists());

    // It should contain the default configuration.
    assert_eq!(config, Config::default());
}
