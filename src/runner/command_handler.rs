use std::process::Command;

use anyhow::{bail, Context};
use colored::ColoredString;

use crate::{
    party_command::PartyCommand,
    util::{make_message_bright_red, CHECK, CROSS, HOURGLASS},
};

use super::run_report::RunReport;

/// Runs a single command
pub fn handle_single_command(
    counter_str: ColoredString,
    party_cmd: &PartyCommand,
) -> anyhow::Result<RunReport> {
    println!("{} {} {}", HOURGLASS, counter_str, party_cmd);

    let mut command = Command::new("sh");
    command.arg("-c").arg(&party_cmd.command);

    if let Some(env_vars) = &party_cmd.env {
        command.envs(env_vars);
    }

    let mut process = command
        .spawn()
        .context(format!("Failed to start task: \"{}\"", party_cmd))?;

    let output = process
        .wait()
        .context(format!("Task failed: \"{}\"", party_cmd))?;

    if !output.success() {
        match output.code() {
            Some(code) => {
                let err_msg = make_message_bright_red(&format!(" returned with code {}", code));
                let full_err_msg = format!("{} {} {} {}", CROSS, counter_str, party_cmd, err_msg);
                eprintln!("{}", full_err_msg);

                return Ok(RunReport::new_failed(full_err_msg));
            }
            None => bail!("Task \"{}\" terminated with a signal", party_cmd),
        }
    }

    let message = format!("{} {} {}", CHECK, counter_str, party_cmd);
    println!("{}", message);

    Ok(RunReport::new_success(message))
}
