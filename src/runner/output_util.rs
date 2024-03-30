use colored::ColoredString;
use colored::Colorize;

pub(super) fn make_counter_blue(step: usize, out_of: usize) -> ColoredString {
    let counter = format!("[{}/{}]", step, out_of);
    counter.blue()
}

pub(super) fn make_eror_message_red(message: String) -> ColoredString {
    message.bright_red()
}
