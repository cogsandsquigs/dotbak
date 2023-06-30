#![cfg(test)]

use crate::{
    errors::{git::GitError, io::IoError, DotbakError},
    Dotbak,
};
use assert_fs::TempDir;
use gix::url::Url;

/// Test if we can create a new repository at a given path.
#[test]
fn test_init_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    println!("{:?}", repo_dir);

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());
}

/// Test if we can create a new repository at a given path that doesn't exist.
#[test]
fn test_init_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let _repo = Dotbak::init_repo(&repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());
}

/// Test if we fail when initing a repo in a repository that already exists.
#[test]
fn test_init_repo_path_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());

    // Try to initialize the repository again.
    let result = Dotbak::init_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    let err = result.unwrap_err();

    println!("{}", err);

    // Check that it is a git init error.
    assert!(matches!(
        err,
        DotbakError::Git {
            source: GitError::Init { .. }
        }
    ));
}

/// Test if we can clone a remote repository into a given path.
#[test]
fn test_clone_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::clone_repo(
        repo_dir,
        Url::from_bytes("https://github.com/cogsandsquigs/dotbak".into()).unwrap(),
    )
    .unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());

    // Check if the README.md file exists.
    assert!(repo_dir.join("README.md").exists());

    // Check if the LICENSE file exists.
    assert!(repo_dir.join("LICENSE").exists());

    // Check if the .gitignore file exists.
    assert!(repo_dir.join(".gitignore").exists());
}

/// Test if we can clone a remote repository into a given path that doesn't exist.
#[test]
fn test_clone_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let _repo = Dotbak::clone_repo(
        &repo_dir,
        Url::from_bytes("https://github.com/cogsandsquigs/dotbak".into()).unwrap(),
    )
    .unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());

    // Check if the README.md file exists.
    assert!(repo_dir.join("README.md").exists());

    // Check if the LICENSE file exists.
    assert!(repo_dir.join("LICENSE").exists());

    // Check if the .gitignore file exists.
    assert!(repo_dir.join(".gitignore").exists());
}

/// Test if we fail when cloning a repo into a repository that already exists.
#[test]
fn test_clone_repo_path_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo: Result<gix::Repository, DotbakError> = Dotbak::init_repo(repo_dir);

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());

    // Try to clone the repository again.
    // THIS SHOULD PANIC
    let result = Dotbak::clone_repo(
        repo_dir,
        Url::from_bytes("https://github.com/cogsandsquigs/dotbak".into()).unwrap(),
    );

    // Check if the result is an error.
    assert!(result.is_err());

    let err = result.unwrap_err();

    println!("{}", err);

    // Check that it is a git clone error.
    assert!(matches!(
        err,
        DotbakError::Git {
            source: GitError::Clone { .. }
        }
    ));
}

/// Test the deletion of a repository.
#[test]
fn test_delete_repo() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let _repo = Dotbak::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(repo_dir.exists());

    // Check if the .git folder exists.
    assert!(repo_dir.join(".git").exists());

    // Delete the repository.
    Dotbak::delete_repo(repo_dir).unwrap();

    // Check if the repository exists.
    assert!(!repo_dir.exists());
}

/// Test the deletion of a repository that doesn't exist.
#[test]
fn test_delete_repo_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Delete the repository.
    let result = Dotbak::delete_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    let err = result.unwrap_err();

    println!("{}", err);

    // Check that it is an IO error.
    assert!(matches!(
        err,
        DotbakError::Io {
            source: IoError::Delete { .. }
        }
    ));
}
