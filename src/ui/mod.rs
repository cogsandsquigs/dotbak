pub mod messages;

use console::{style, Term};
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

const SPINNER_FRAMES: &[&str] = &[
    "⠁", "⠂", "⠄", "⡀", "⡈", "⡐", "⡠", "⣀", "⣁", "⣂", "⣄", "⣌", "⣔", "⣤", "⣥", "⣦", "⣮", "⣶", "⣷",
    "⣿", "⡿", "⠿", "⢟", "⠟", "⡛", "⠛", "⠫", "⢋", "⠋", "⠍", "⡉", "⠉", "⠑", "⠡", "⢁",
];

const SPINNER_FRAME_DURATION: Duration = Duration::from_millis(80);

/// An interface to the terminal, for spinners. This is a wrapper around `indicatif::MultiProgress`, and also is
/// `Clone`-able (as it uses Rc internally).
#[derive(Clone, Debug)]
pub struct Interface {
    /// The terminal to draw to.
    term: Term,

    /// The multi-progress bar.
    mp: MultiProgress,

    /// The largest spinner message length.
    max_msg_len: usize,

    /// The current spinner "depth"
    current_depth: usize,
}

impl Interface {
    /// Creates a new interface.
    pub fn new(max_msg_len: usize) -> Interface {
        let term = Term::stdout();
        let draw_target = ProgressDrawTarget::term(term.clone(), 30);

        Interface {
            mp: MultiProgress::with_draw_target(draw_target),
            term,
            max_msg_len,
            current_depth: 0,
        }
    }

    /// Draw a message to the terminal.
    pub fn println<S>(&self, message: S)
    where
        S: ToString,
    {
        self.term.write_line(&message.to_string()).unwrap();
    }

    /// Print a warning to the terminal.
    pub fn warn<S>(&self, message: S)
    where
        S: ToString,
    {
        self.term
            .write_line(
                &style(format!("❗️ {}", message.to_string()))
                    .yellow()
                    .to_string(),
            )
            .unwrap();
    }

    /// Spawns a new spinner. Returns a handle to the spinner, which can be used to update the spinner.
    pub fn spawn_spinner<S>(&mut self, message: S, depth: usize) -> Spinner
    where
        S: ToString,
    {
        let message = message.to_string();
        let num_dots = self.max_msg_len.saturating_sub(message.len());

        let new_depth = depth > self.current_depth;
        self.current_depth = depth;

        let pb = ProgressBar::new_spinner().with_message(message).with_style(
            ProgressStyle::default_spinner()
                .template(&get_template("{spinner:.blue}", num_dots, depth, new_depth))
                .expect("This should not fail!")
                .tick_strings(SPINNER_FRAMES),
        );

        let mut spinner = Spinner::new(self.mp.add(pb), num_dots, self.current_depth, new_depth);

        spinner.start();

        spinner
    }
}

/// A wrapper around a progress bar.
#[derive(Clone, Debug)]
pub struct Spinner {
    /// The underlying progress bar.
    spinner: ProgressBar,

    /// The number of dots to display after the message.
    num_dots: usize,

    /// The depth of the spinner.
    depth: usize,

    /// Whether the spinner was created with a new depth.
    new_depth: bool,
}

impl Spinner {
    pub fn new(spinner: ProgressBar, num_dots: usize, depth: usize, new_depth: bool) -> Spinner {
        Spinner {
            spinner,
            num_dots,
            depth,
            new_depth,
        }
    }

    /// Starts the spinner. Note that the spinner does not appear until the first tick.
    pub fn start(&mut self) {
        self.spinner.enable_steady_tick(SPINNER_FRAME_DURATION);
    }

    /// Closes the spinner.
    pub fn close(self) {
        let raw_spinner = self.spinner;

        raw_spinner.set_style(
            ProgressStyle::default_spinner()
                .template(&get_template(
                    &console::style("✓").green().to_string(),
                    self.num_dots,
                    self.depth,
                    self.new_depth,
                ))
                .expect("This should not fail!")
                .tick_strings(SPINNER_FRAMES),
        );

        raw_spinner.finish();
    }
}

fn get_template(ending: &str, num_dots: usize, depth: usize, new_depth: bool) -> String {
    let depth_string = if new_depth {
        "   ".repeat(depth) + "╰─→ "
    } else if depth > 0 {
        "   ".repeat(depth) + "    "
    } else {
        "   ".into()
    };

    let num_dots = if depth > 0 {
        num_dots.saturating_sub(3 * depth + 1)
    } else {
        num_dots
    };

    format!(
        "{tabs}{{msg}} {dots} {ending}",
        tabs = console::style(&depth_string).dim(),
        dots = console::style("·".repeat(num_dots)).dim(),
    )
}
