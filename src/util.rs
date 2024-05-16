//! Helper functions for priniting information.
use std::path::Path;

use colored::ColoredString;
use colored::Colorize;

/// Default party configuration file to read from if available
pub const DEFAULT_PARTY_CONF: &str = "./party.toml";

/// Check if the given file exists
pub fn check_file_path(file_path: &str) -> anyhow::Result<()> {
    if !Path::new(file_path).exists() {
        Err(anyhow::Error::msg(format!(
            "File {} does not exist",
            file_path
        )))
    } else {
        Ok(())
    }
}

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

/// Hourglass unicode
pub const HOURGLASS: char = '\u{23F3}';

/// Cross unicode
pub const CROSS: char = '\u{274C}';

/// Check unicode
pub const CHECK: char = '\u{2705}';

/// Horse unicode
pub const HORSE: char = '\u{1F3C7}';

/// Party popper unicode
pub const POPPER: char = '\u{1F389}';
