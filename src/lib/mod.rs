pub mod config;
pub mod errors;

use config::Config;

/// The main structure to manage the program's actions and such.
pub struct Dotbak {
    /// The configuration for the program.
    pub config: Config,
}
