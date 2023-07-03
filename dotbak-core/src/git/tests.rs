#![cfg(test)]

use crate::{
    errors::{git::GitError, io::IoError, DotbakError},
    git::GitRepo,
    repo_exists, repo_not_exists,
};
use assert_fs::{prelude::*, TempDir};

/// The repository URL for the test repository.
const TEST_GIT_REPO_URL: &str = "https://github.com/githubtraining/hellogitworld";

/// Test if we can create a new repository at a given path.
#[test]
fn test_init_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None).unwrap();

    println!("{:?}", repo_dir);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can create a new repository at a given path that doesn't exist.
#[test]
fn test_init_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = GitRepo::init(&repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we fail when initing a repo in a repository that already exists.
#[test]
fn test_init_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Try to initialize the repository again.
    let result = GitRepo::init(repo_dir, None);

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
fn test_load_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);

    // Load the repository.
    let result = GitRepo::load(repo_dir);

    // Check if the result is ok.
    assert!(result.is_ok());

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can load a pre-existing repository that doesn't exist.
#[test]
fn test_load_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Load the repository.
    let result = GitRepo::load(repo_dir);

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
fn test_clone_path_exists() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::clone(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we can clone a remote repository into a given path that doesn't exist.
#[test]
fn test_clone_path_nonexistent() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path().join("some/sub/folders");

    // Initialize the repository.
    let repo = GitRepo::clone(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test if we fail when cloning a repo into a repository that already exists.
#[test]
fn test_clone_exists_already() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None);

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.unwrap().path, repo_dir);

    // Try to clone the repository again.
    // THIS SHOULD PANIC
    let result = GitRepo::clone(repo_dir, TEST_GIT_REPO_URL);

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

/// Test if we can run arbitrary commands in a repository.
/// This just does `git add .` and `git commit -m "Test commit"`.
#[test]
fn test_arbitrary_command() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Set username and email. This is so that we can commit in the CI.
    repo.arbitrary_command(&["config", "user.name", "Test User"])
        .unwrap();
    repo.arbitrary_command(&["config", "user.email", "test_user@tests"])
        .unwrap();

    // Create a file in the repository.
    tmp_dir.child("test.txt").touch().unwrap();

    // Run the arbitrary command.
    repo.arbitrary_command(&["add", "."]).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());

    // Run the arbitrary command.
    repo.arbitrary_command(&["commit", "-m", "Test commit"])
        .unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());
}

/// Test if we can commit changes to a repository.
#[test]
fn test_commit() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let mut repo = GitRepo::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Create a file in the repository.
    tmp_dir.child("test.txt").touch().unwrap();

    // Commit the changes.
    repo.commit("Initial commit").unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());

    // Create a new file in the repository.
    tmp_dir.child("test2.txt").touch().unwrap();

    // Commit the changes.
    repo.commit("Second commit").unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);
    assert!(tmp_dir.child("test.txt").path().exists());
    assert!(tmp_dir.child("test2.txt").path().exists());
}

/// Test setting the remote of a repository.
#[test]
fn test_set_remote() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let mut repo_dir = tmp_dir.path().to_path_buf();

    // Initialize the repository.
    let mut repo = GitRepo::init(&repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Set the remote.
    repo.set_remote(TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Clone the repository.
    repo_dir = tmp_dir.path().join("clone");
    let mut repo = GitRepo::clone(&repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Set the remote.
    repo.set_remote(TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(&repo_dir);
    assert_eq!(repo.path, repo_dir);
}

/// Test pushing data to a remote repository.
#[test]
fn test_push() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let mut repo = GitRepo::clone(repo_dir, TEST_GIT_REPO_URL).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Push the changes.
    repo.push().unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path(), repo_dir);
}

/// Test the deletion of a repository.
#[test]
fn test_delete() {
    // Create a temporary directory.
    let tmp_dir = TempDir::new().unwrap();

    // Get the path to the repo directory.
    let repo_dir = tmp_dir.path();

    // Initialize the repository.
    let repo = GitRepo::init(repo_dir, None).unwrap();

    // Check if the repository exists.
    repo_exists!(repo_dir);
    assert_eq!(repo.path, repo_dir);

    // Delete the repository.
    repo.delete().unwrap();

    // Check if the repository exists.
    repo_not_exists!(repo_dir);
}
