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

    let test_file = PathBuf::from("test.txt");
    let full_test_file_path = home_dir.join(&test_file);
    let expected_file = repo_dir.join("test.txt");

    // Create the home directory and the test file.
    fs::create_dir_all(&home_dir).unwrap();
    assert!(home_dir.exists());
    assert!(!full_test_file_path.exists());
    fs::File::create(&full_test_file_path).unwrap();

    assert!(full_test_file_path.exists());

    let mut dotbak = Dotbak::init_into_dirs(&home_dir, config_file, repo_dir).unwrap();

    assert!(!dotbak.config.files.include.contains(&test_file));
    assert!(!expected_file.exists());

    dotbak.add(&[&test_file]).unwrap();

    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert_eq!(full_test_file_path.read_link().unwrap(), expected_file);
    assert!(dotbak.config.files.include.contains(&test_file));
    assert!(expected_file.exists());
}

/// Test if we can implicitly add a folder's contents.
#[test]
fn test_add_folder() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");

    let test_folder = PathBuf::from("test");
    let test_file = PathBuf::from("test/test.txt");
    let full_test_folder_path = home_dir.join(&test_folder);
    let full_test_file_path = home_dir.join(test_file);
    let expected_folder = repo_dir.join("test");
    let expected_file = repo_dir.join("test/test.txt");

    // Create the home directory and the test folder and file.
    fs::create_dir_all(&home_dir).unwrap();
    assert!(home_dir.exists());
    fs::create_dir_all(&full_test_folder_path).unwrap();
    fs::File::create(&full_test_file_path).unwrap();

    assert!(full_test_folder_path.exists());
    assert!(full_test_file_path.exists());

    let mut dotbak = Dotbak::init_into_dirs(&home_dir, config_file, repo_dir).unwrap();

    assert!(!dotbak.config.files.include.contains(&test_folder));
    assert!(!expected_folder.exists());

    dotbak.add(&[&test_folder]).unwrap();

    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert_eq!(full_test_folder_path.read_link().unwrap(), expected_folder);
    assert!(dotbak.config.files.include.contains(&test_folder));
    assert!(expected_folder.exists());
    assert!(expected_file.exists());
}

/// Test that we can remove files after adding them to the `Dotbak` manager.
#[test]
fn test_remove_files() {
    let dir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");

    let test_file = PathBuf::from("test.txt");
    let full_test_file_path = home_dir.join(&test_file);
    let expected_file = repo_dir.join("test.txt");

    // Create the home directory and the test file.
    fs::create_dir_all(&home_dir).unwrap();
    assert!(home_dir.exists());
    assert!(!full_test_file_path.exists());
    fs::File::create(&full_test_file_path).unwrap();

    assert!(full_test_file_path.exists());

    let mut dotbak = Dotbak::init_into_dirs(&home_dir, config_file, repo_dir).unwrap();

    assert!(!dotbak.config.files.include.contains(&test_file));
    assert!(!expected_file.exists());

    dotbak.add(&[&test_file]).unwrap();

    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert_eq!(full_test_file_path.read_link().unwrap(), expected_file);
    assert!(dotbak.config.files.include.contains(&test_file));
    assert!(expected_file.exists());

    dotbak.remove(&[&test_file]).unwrap();

    assert!(!dotbak.config.files.include.contains(&test_file));
    assert!(!expected_file.exists());
    assert!(full_test_file_path.exists());
}

/// Test if we can deinitialize a `Dotbak` instance after adding files to it.
#[test]
fn test_delete_dotbak() {
    let dir: TempDir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let mut dotbak = Dotbak::init_into_dirs(&home_dir, &config_file, &repo_dir).unwrap();

    // Clear the include list (because it links out of the test directory)
    dotbak.config.files.include = vec![];

    let test_file = PathBuf::from("test.txt");
    let full_test_file_path = home_dir.join(&test_file);
    let expected_file = repo_dir.join("test.txt");

    // Create the home directory and the test file.
    fs::create_dir_all(&home_dir).unwrap();
    assert!(home_dir.exists());
    assert!(!full_test_file_path.exists());
    fs::File::create(&full_test_file_path).unwrap();

    assert!(full_test_file_path.exists());
    assert!(!dotbak.config.files.include.contains(&test_file));
    assert!(!expected_file.exists());

    dotbak.add(&[&test_file]).unwrap();

    // This is a symlink, so instead of checking if it exists, check if it's a symlink.
    assert_eq!(full_test_file_path.read_link().unwrap(), expected_file);
    assert!(dotbak.config.files.include.contains(&test_file));
    assert!(expected_file.exists());

    dotbak.deinit().unwrap();

    assert!(!expected_file.exists());
    assert!(!config_file.exists());
    assert!(!repo_dir.exists());
    assert!(full_test_file_path.exists());
}

/// Test if we can synchronize all the files. I.e., if we can add files that are not symlinked but in
/// the repository, and replace files that are already there.
#[test]
fn test_sync_all_files() {
    let dir: TempDir = TempDir::new().unwrap();
    let home_dir = dir.path().join("home");
    let config_file = dir.path().join("config.toml");
    let repo_dir = dir.path().join("repo");
    let mut dotbak = Dotbak::init_into_dirs(&home_dir, config_file, &repo_dir).unwrap();

    let test_file_1 = PathBuf::from("test.txt");
    let test_file_2 = PathBuf::from("test2.txt");

    dotbak.config.files.include = vec![test_file_1.clone(), test_file_2.clone()];

    let full_test_file_path_1 = repo_dir.join(&test_file_1);
    let full_test_file_path_2 = repo_dir.join(&test_file_2);

    let expected_file_1 = home_dir.join("test.txt");
    let expected_file_2 = home_dir.join("test2.txt");

    // Create the home directory and the test file.
    fs::create_dir_all(&home_dir).unwrap();

    assert!(home_dir.exists());
    assert!(!expected_file_1.exists());
    assert!(!expected_file_2.exists());

    fs::File::create(&full_test_file_path_1).unwrap();
    fs::File::create(&full_test_file_path_2).unwrap();
    fs::File::create(&expected_file_2).unwrap();

    // Write content to the second file
    fs::write(&full_test_file_path_2, "test").unwrap();

    // Write content to the dummy file
    fs::write(&expected_file_2, "dummy").unwrap();

    assert!(full_test_file_path_1.exists());
    assert!(full_test_file_path_2.exists());
    assert!(!expected_file_1.exists());
    assert!(expected_file_2.exists());
    assert!(dotbak.config.files.include.contains(&test_file_1));
    assert!(dotbak.config.files.include.contains(&test_file_2));
    assert_eq!(fs::read_to_string(&expected_file_2).unwrap(), "dummy");

    dotbak.sync_all_files().unwrap();

    assert!(full_test_file_path_1.exists());
    assert!(full_test_file_path_2.exists());
    assert!(expected_file_1.exists());
    assert!(expected_file_2.exists());
    assert!(dotbak.config.files.include.contains(&test_file_1));
    assert!(dotbak.config.files.include.contains(&test_file_2));
    assert_eq!(fs::read_to_string(&expected_file_2).unwrap(), "test");
}
