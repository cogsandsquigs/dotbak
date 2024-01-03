use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(100);

/// A wrapper around a progress bar.
#[derive(Clone, Debug)]
pub struct Spinner {
    spinner: ProgressBar,
    pre_msg_pad: String,
    post_msg_elipses: usize,
    current: usize,
    total: usize,
}

impl Spinner {
    /// Creates a new spinner.
    pub fn new(
        message: String,
        pre_msg_pad: &str,
        post_msg_elipses: usize,
        current: usize,
        total: usize,
    ) -> Spinner {
        Spinner {
            spinner: ProgressBar::new_spinner()
                .with_message(message)
                // Default spinner style
                .with_style(
                    ProgressStyle::default_spinner()
                        .template(&template_with_ending(
                            "{spinner:.cyan/blue}",
                            pre_msg_pad,
                            post_msg_elipses,
                            current,
                            total,
                        ))
                        .expect("This should not fail!")
                        .tick_strings(SPINNER_FRAMES),
                ),
            pre_msg_pad: pre_msg_pad.to_string(),
            post_msg_elipses,
            current,
            total,
        }
    }

    /// Starts the spinner. Note that the spinner does not appear until the first tick.
    pub fn start(&mut self) {
        self.spinner.enable_steady_tick(SPINNER_FRAME_DURATION);
    }

    /// Closes the spinner.
    pub fn close(self) {
        self.spinner.set_style(
            ProgressStyle::default_spinner()
                .template(&template_with_ending(
                    "✅",
                    &self.pre_msg_pad,
                    self.post_msg_elipses,
                    self.current,
                    self.total,
                ))
                .expect("This should not fail!")
                .tick_strings(SPINNER_FRAMES),
        );

        self.spinner.tick();

        self.spinner.finish();
    }
}

/// Creates a template with an attached spinner count and padding, with a custom ending.
fn template_with_ending(
    ending: &str,
    pre_msg_pad: &str,
    post_msg_elipses: usize,
    current: usize,
    total: usize,
) -> String {
    format!(
        "{}{} {{msg}} ...{} {}",
        pre_msg_pad,
        console::style(format!("[{}/{}]", current, total))
            .bold()
            .dim(),
        ".".repeat(post_msg_elipses),
        ending,
    )
}
