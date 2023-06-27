pub mod config;
pub mod errors;

use config::Config;
use errors::Result;

/// The main structure to manage the program's actions and such.
pub struct Dotbak {
    /// The configuration for the program.
    pub config: Config,
}

/// Public API for the program.
impl Dotbak {
    /// Create a new instance of the program. This also automatically loads the configuration.
    pub fn new() -> Result<Self> {
        let config = Config::load_config()?;

        Ok(Dotbak { config })
    }
}
