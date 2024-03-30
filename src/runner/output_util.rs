//! Helper functions for priniting information.
use colored::ColoredString;
use colored::Colorize;

use super::run_report::RunReport;

/// Create blue counter message
pub fn make_counter_blue(step: usize, out_of: usize) -> ColoredString {
    let counter = format!("[{}/{}]", step, out_of);
    counter.blue()
}

/// Create a red error message
pub fn make_eror_message_red(message: String) -> ColoredString {
    message.bright_red()
}

/// Display the status report for the current run
pub fn print_status_report(failed: usize, total: usize, reports: Vec<RunReport>) {
    if failed != 0 {
        println!("\ncargo party report - {}/{} failed tasks:", failed, total);
    } else {
        println!("\ncargo party report - all tasks passed:");
    }
    for report in reports {
        if report.success {
            println!("{}", report.message);
        } else {
            eprintln!("{}", report.message);
        }
    }
    println!();
}
