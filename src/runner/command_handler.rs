use std::process::Command;

use anyhow::{bail, Context};
use colored::ColoredString;

use crate::{
    party_command::PartyCommand,
    util::{make_message_bright_red, CHECK, CROSS, HOURGLASS},
};

use super::run_report::RunReport;

pub fn handle_single_command(
    counter_str: ColoredString,
    raw_cmd: &PartyCommand,
) -> anyhow::Result<RunReport> {
    println!("{} {} {}", HOURGLASS, counter_str, raw_cmd);

    let mut command: std::process::Child = Command::new(raw_cmd.command.clone())
        .args(raw_cmd.args.clone())
        .spawn()
        .context(format!("Failed to start task: \"{}\"", raw_cmd))?;

    let output = command
        .wait()
        .context(format!("Task failed: \"{}\"", raw_cmd))?;

    if !output.success() {
        match output.code() {
            Some(code) => {
                let err_msg = make_message_bright_red(&format!(" returned with code {}", code));
                let full_err_msg = format!("{} {} {} {}", CROSS, counter_str, raw_cmd, err_msg);
                eprintln!("{}", full_err_msg);

                return Ok(RunReport::new_failed(full_err_msg));
            }
            None => bail!("Task \"{}\" terminated with a signal", raw_cmd),
        }
    }

    let message = format!("{} {} {}", CHECK, counter_str, raw_cmd);
    println!("{}", message);

    Ok(RunReport::new_success(message))
}
