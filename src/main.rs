pub mod logger;

use logger::{LogLevel, Logger};
use std::io;

fn main() -> io::Result<()> {
    let logger = Logger::new(LogLevel::Debug);

    logger.debug("Test!")?;
    logger.info("Test!")?;
    logger.warn("Test!")?;
    logger.error("Test!")?;

    Ok(())
}
