//! Helper functions for priniting information.
use colored::ColoredString;
use colored::Colorize;

/// Create blue counter message
pub fn make_counter_blue(step: usize, out_of: usize) -> ColoredString {
    let counter = format!("[{}/{}]", step, out_of);
    counter.blue()
}

/// Create a bright green message
pub fn make_message_bright_green(message: &str) -> ColoredString {
    message.green()
}

/// Create a bright red message
pub fn make_message_bright_red(message: &str) -> ColoredString {
    message.bright_red()
}
