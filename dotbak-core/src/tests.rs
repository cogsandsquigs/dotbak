#![cfg(test)]

use super::*;
use assert_fs::TempDir;
use std::{fs, path::PathBuf};

/// The repository URL for the test repository.
const TEST_GIT_REPO_URL: &str = "https://github.com/cogsandsquigs/dotbak";

/// Test if we can initialize a new `Dotbak` instance from a directory.
#[test]
fn test_init_dotbak() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::init_into_dirs(home_dir, &config_file, &repo_dir);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().repo.path(), repo_dir);
    assert!(config_file.exists());
    repo_exists!(repo_dir);
}

/// Test if we can initialize a new `Dotbak` instance from a directory that does not exist.
#[test]
fn test_init_dotbak_no_dir() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::init_into_dirs(home_dir, &config_file, &repo_dir);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().repo.path(), repo_dir);
    assert!(config_file.exists());
    repo_exists!(repo_dir);
}

/// Test if we can load an existing `Dotbak` instance from a directory with a configuration file
/// and a repository.
#[test]
fn test_load_dotbak() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::init_into_dirs(&home_dir, &config_file, &repo_dir);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().repo.path(), repo_dir);
    assert!(config_file.exists());
    repo_exists!(repo_dir);

    let result = Dotbak::load_into_dirs(home_dir, &config_file, &repo_dir);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().repo.path(), repo_dir);
    assert!(config_file.exists());
    repo_exists!(repo_dir);
}

/// Test if we can load an existing `Dotbak` instance from a directory that has not yet been initialized.
#[test]
fn test_load_dotbak_no_dir() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::load_into_dirs(home_dir, config_file, repo_dir);

    assert!(result.is_err());
    assert!(matches!(
        result.err().unwrap(),
        DotbakError::Config {
            source: ConfigError::ConfigNotFound { .. },
        }
    ));
}

/// Test if we can clone a repository from a remote location and initialize a `Dotbak` instance from it.
#[test]
fn test_clone_dotbak() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::clone_into_dirs(home_dir, &config_file, &repo_dir, TEST_GIT_REPO_URL);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().repo.path(), repo_dir);
    assert!(config_file.exists());
    repo_exists!(repo_dir);
}

/// Test if we can add files to the `Dotbak` manager.
#[test]
fn test_add_files() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let result = Dotbak::init_into_dirs(&home_dir, &config_file, &repo_dir);

    assert!(result.is_ok());

    let test_file = PathBuf::from("test.txt");
    let full_test_file_path = home_dir.join(&test_file);
    let expected_file = repo_dir.join("test.txt");

    // Create the home directory and the test file.
    fs::create_dir_all(&home_dir).unwrap();
    assert!(home_dir.exists());
    assert!(!full_test_file_path.exists());
    fs::File::create(&full_test_file_path).unwrap();

    assert!(full_test_file_path.exists());

    let mut dotbak = Dotbak::init_into_dirs(&home_dir, &config_file, repo_dir).unwrap();

    assert!(!dotbak.config.files.include.contains(&test_file));
    assert!(!expected_file.exists());

    dotbak.add(&[&test_file]).unwrap();

    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert_eq!(full_test_file_path.read_link().unwrap(), expected_file);
    assert!(dotbak.config.files.include.contains(&test_file));
    assert!(expected_file.exists());
}

// /// Test if we can exclude files/dirs from the `Dotbak` manager.
// #[test]
// fn test_exclude() {
//     let dir = TempDir::new().unwrap();
//     let home_dir = dir.path().join("home");
//     let dotbak_dir = dir.path().join("dotbak");

//     let test_files = vec![
//         // To include...
//         PathBuf::from("foo"),
//         PathBuf::from("baz/quz"),
//         // To exclude...
//         PathBuf::from("bar"),
//         PathBuf::from("baz/spam"),
//     ];

//     let full_test_file_paths = test_files
//         .iter()
//         .map(|file| home_dir.join(file))
//         .collect_vec();

//     let expected_files = vec![PathBuf::from("foo"), PathBuf::from("baz/quz")];

//     // Create the home directory and the test file.
//     fs::create_dir_all(&home_dir).unwrap();
//     assert!(home_dir.exists());
//     assert!(full_test_file_paths.iter().all(|path| !path.exists()));

//     for path in &full_test_file_paths {
//         fs::create_dir_all(path.parent().unwrap()).unwrap();
//         fs::File::create(path).unwrap();
//     }

//     let mut dotbak = Dotbak::init_into_dirs(&home_dir, dotbak_dir).unwrap();

//     assert!(expected_files
//         .iter()
//         .all(|file| !dotbak.config.files.include.contains(file)));

//     dotbak.add(&test_files).unwrap();

//     assert!(expected_files
//         .iter()
//         .all(|file| dotbak.config.files.include.contains(file)));

//     // Exclude `bar` and `baz/spam`.
//     dotbak.exclude(&test_files[2..4]).unwrap();

//     assert!(expected_files
//         .iter()
//         .all(|file| !dotbak.config.files.include.contains(file)));
//     assert!(test_files[2..4]
//         .iter()
//         .all(|file| dotbak.config.files.exclude.contains(file)));

//     // // This is a symlink, so instead of checking if it exists, check if it's a symlink.
//     // assert_eq!(full_test_file_path.read_link().unwrap(), expected_file);
//     // assert!(dotbak.config.files.include.contains(&test_file));
//     // assert!(expected_file.exists());
// }
