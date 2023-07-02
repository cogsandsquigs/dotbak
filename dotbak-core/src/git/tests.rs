#![cfg(test)]

use crate::{
    errors::{git::GitError, io::IoError, DotbakError},
    git::GitRepo,
    repo_exists, repo_not_exists,
};
use assert_fs::TempDir;

/// The repository URL for the test repository.
const TEST_GIT_REPO_URL: &str = "https://github.com/githubtraining/hellogitworld";

/// Test if we can create a new repository at a given path.
#[test]
fn test_init_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init_repo(repo_dir).unwrap();

    println!("{:?}", repo_dir);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can create a new repository at a given path that doesn't exist.
#[test]
fn test_init_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = GitRepo::init_repo(&repo_dir).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we fail when initing a repo in a repository that already exists.
#[test]
fn test_init_repo_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Try to initialize the repository again.
    let result = GitRepo::init_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is a git init error.
    assert!(matches!(
        result,
        Err(DotbakError::Git {
            source: GitError::Init { .. }
        })
    ));
}

/// Test if we can load a pre-existing repository.
#[test]
fn test_load_repo_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);

    // Load the repository.
    let result = GitRepo::load_repo(repo_dir);

    // Check if the result is ok.
    assert!(result.is_ok());

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can load a pre-existing repository that doesn't exist.
#[test]
fn test_load_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Load the repository.
    let result = GitRepo::load_repo(repo_dir);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is an IO error.
    assert!(matches!(
        result,
        Err(DotbakError::Io {
            source: IoError::NotFound { .. }
        })
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
    let repo = GitRepo::clone_repo(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can clone a remote repository into a given path that doesn't exist.
#[test]
fn test_clone_repo_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = GitRepo::clone_repo(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we fail when cloning a repo into a repository that already exists.
#[test]
fn test_clone_repo_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init_repo(repo_dir);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.unwrap().path, repo_dir);

    // Try to clone the repository again.
    // THIS SHOULD PANIC
    let result = GitRepo::clone_repo(repo_dir, TEST_GIT_REPO_URL);

    // Check if the result is an error.
    assert!(result.is_err());

    // Check that it is a git clone error.
    assert!(matches!(
        result,
        Err(DotbakError::Git {
            source: GitError::Clone { .. }
        })
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
    let mut repo = GitRepo::init_repo(repo_dir).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Delete the repository.
    repo.delete_repo().unwrap();

    // Check if the repository exists.
    repo_not_exists!(repo_dir);
}
