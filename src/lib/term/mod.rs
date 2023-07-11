use std::time::Duration;

use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};

const SPINNER_FRAMES: &[&str] = &["◜", "◠", "◝", "◞", "◡", "◟"];
// const SPINNER_FRAMES: &[&str] = &["○", "◔", "◑", "◕", "●", "◟"];

const SPINNER_DURATION: Duration = Duration::from_millis(100);

/// Creates a new spinner with the given message.
pub fn new_spinner(message: &'static str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner()
        .with_message(message)
        .with_style(ProgressStyle::default_spinner().tick_strings(SPINNER_FRAMES))
        .with_finish(ProgressFinish::AndClear);

    spinner.enable_steady_tick(SPINNER_DURATION);

    spinner
}
