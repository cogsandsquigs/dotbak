use indicatif::{ProgressBar, ProgressStyle};
use std::{borrow::Cow, time::Duration};

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(100);

/// Create the default spinner.
pub fn create_spinner<S>(message: S) -> ProgressBar
where
    S: Into<Cow<'static, str>>,
{
    let spinner = ProgressBar::new_spinner()
        .with_message(message)
        .with_style(spinner_style());

    spinner.enable_steady_tick(SPINNER_FRAME_DURATION);

    spinner
}

/// Gets the spinner style.
fn spinner_style() -> ProgressStyle {
    ProgressStyle::default_spinner()
        .template("{spinner:.cyan/blue} {msg} [{elapsed}]")
        .expect("This should not fail!")
        .tick_strings(SPINNER_FRAMES)
}
