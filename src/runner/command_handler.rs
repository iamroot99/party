use std::process::Command;

use anyhow::{bail, Context};
use colored::ColoredString;

use crate::{party_command::PartyCommand, runner::output_util::make_eror_message_red};

use super::run_report::RunReport;

pub fn handle_single_command(
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
                let err_msg = make_eror_message_red(format!(" returned with code {}!", code));
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
