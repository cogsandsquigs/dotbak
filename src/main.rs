mod config;

use config::Configuration;
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let dotback = Dotback::new()?;

    Ok(())
}

/// Manages dotfiles, given a certain configuration location.
struct Dotback {
    /// The path to the dotback directory, where the configuration and dotfile repository
    /// lies.
    dir: PathBuf,

    /// The configuration for this instance.
    config: Configuration,
}

impl Dotback {
    /// Creates a new `Dotback` instance, which can be used to manage dotfiles.
    pub fn new() -> io::Result<Self> {
        let path = PathBuf::from(if let Ok(path) = env::var("DOTBACK_DIR") {
            path
        } else {
            "~/.dotback".into()
        });

        let dotback = Self {
            dir: path,
            config: Configuration::default(),
        };

        let config_file = File::create(&dotback.config_path())?;

        // If the config file is empty, we know it is newly created and therefore we just write
        // the configuration to said file.
        todo!();

        Ok(dotback)
    }

    fn init(&mut self) -> io::Result<()> {
        if self.config_path().exists() {
            println!("Configuration already exists at {}", self.dir.display());

            if let Ok(config) = toml::from_str(&fs::read_to_string(self.config_path())?) {
                self.config = config;
            } else {
                println!("Configuration is erroneous or corrupted!");
                println!("Overriding with defaults...");

                self.write_config()?;
            }

            return Ok(());
        }

        println!(
            "Initializing configuration at {}",
            self.config_path().display()
        );

        // TODO: ask user for repository location
        self.config.repository = todo!();

        self.write_config()?;

        Ok(())
    }
}

/// Private API for `Dotback`
impl Dotback {
    /// Gets the path to the configuration.
    fn config_path(&self) -> PathBuf {
        self.dir.join("config.toml")
    }

    /// Writes the configuration to the config file.
    fn write_config(&self) -> io::Result<()> {
        let mut file = if self.config_path().exists() {
            File::open(self.config_path())
        } else {
            fs::create_dir_all(&self.dir)?;

            File::create(self.config_path())
        }?;

        file.write_all(
            toml::to_string(&self.config)
                .expect("This should never panic!")
                .as_bytes(),
        )?;

        Ok(())
    }
}
