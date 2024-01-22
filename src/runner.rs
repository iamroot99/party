//! Main command runner
use crate::{party_command::PartyCommand, schdeuler::CommandBatch};
use anyhow::{bail, Context, Ok};
use colored::{ColoredString, Colorize};
use std::process::Command;

/// Main driver function running all the party commands in batches.
pub async fn run_commands(batches: Vec<CommandBatch>, no_commands: usize) -> anyhow::Result<()> {
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
            handle.await.context("Failed to join concurrent task")??;
        }
    }

    Ok(())
}

fn handle_single_command(counter_str: ColoredString, raw_cmd: &PartyCommand) -> anyhow::Result<()> {
    println!("⏳ {} {}", counter_str, raw_cmd);

    let mut command: std::process::Child = Command::new(raw_cmd.command.clone())
        .args(raw_cmd.args.clone())
        .spawn()
        .context(format!("Failed to start command: \"{}\"", raw_cmd))?;

    let rc = command
        .wait()
        .context(format!("Command failed: \"{}\"", raw_cmd))?;

    if !rc.success() {
        match rc.code() {
            Some(code) => {
                let err_msg = make_eror_message(format!(" returned with code {}!", code));
                bail!("❌ {} {} {}", counter_str, raw_cmd, err_msg);
            }
            None => bail!("Command \"{}\" failed with no return code", raw_cmd),
        }
    }

    println!("✅ {} {}\n", counter_str, raw_cmd);
    Ok(())
}

fn make_counter(step: usize, out_of: usize) -> ColoredString {
    let counter = format!("[{}/{}]", step, out_of);
    counter.blue()
}

fn make_eror_message(message: String) -> ColoredString {
    message.bright_red()
}
