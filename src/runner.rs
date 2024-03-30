//! Main command runner
use crate::{party_command::PartyCommand, schdeuler::CommandBatch};
use anyhow::{bail, Context, Ok};
use colored::{ColoredString, Colorize};
use std::process::Command;

struct RunReport {
    pub message: String,
    pub success: bool,
}

impl RunReport {
    pub fn new_success(message: String) -> Self {
        Self {
            message,
            success: true,
        }
    }

    pub fn new_failed(message: String) -> Self {
        Self {
            message,
            success: false,
        }
    }
}

/// Main driver function running all the party commands in batches.
pub async fn run_commands(batches: Vec<CommandBatch>, no_commands: usize) -> anyhow::Result<()> {
    let mut reports: Vec<RunReport> = vec![];
    let mut failed = 0;

    let mut curr: usize = 0;

    for batch in batches {
        let mut handles = vec![];

        for command in batch {
            curr += 1;

            handles.push(tokio::spawn(async move {
                handle_single_command(make_counter(curr, no_commands), &command)
            }));
        }

        for handle in handles {
            let report = handle.await.context("Failed to join concurrent task")??;
            if !report.success {
                failed += 1;
            }

            reports.push(report);
        }
    }

    if failed != 0 {
        println!(
            "\ncargo party report - {}/{} failed tasks:",
            failed, no_commands
        );
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

    Ok(())
}

fn handle_single_command(
    counter_str: ColoredString,
    raw_cmd: &PartyCommand,
) -> anyhow::Result<RunReport> {
    println!("⏳ {} {}", counter_str, raw_cmd);

    let mut command: std::process::Child = Command::new(raw_cmd.command.clone())
        .args(raw_cmd.args.clone())
        .spawn()
        .context(format!("Failed to start command: \"{}\"", raw_cmd))?;

    let output = command
        .wait()
        .context(format!("Command failed: \"{}\"", raw_cmd))?;

    if !output.success() {
        match output.code() {
            Some(code) => {
                let err_msg = make_eror_message(format!(" returned with code {}!", code));
                let full_err_msg = format!("❌ {} {} {}", counter_str, raw_cmd, err_msg);
                eprintln!("{}", full_err_msg);

                return Ok(RunReport::new_failed(full_err_msg));
            }
            None => bail!("Command \"{}\" terminated with a signal", raw_cmd),
        }
    }

    let message = format!("✅ {} {}", counter_str, raw_cmd);
    println!("{}", message);
    Ok(RunReport::new_success(message))
}

fn make_counter(step: usize, out_of: usize) -> ColoredString {
    let counter = format!("[{}/{}]", step, out_of);
    counter.blue()
}

fn make_eror_message(message: String) -> ColoredString {
    message.bright_red()
}
