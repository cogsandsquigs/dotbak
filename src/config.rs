/// The main configuration for dotback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// The git repository where the dotfiles are synced to.
    repository: String,
}
