//! `party run`
use std::path::Path;

use anyhow::bail;

use crate::{
    cli::RunArgs,
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands, PartyCommand},
    runner, schdeuler,
    util::{check_file_path, DEFAULT_PARTY_CONF},
};

/// Implementation of `party run`
pub async fn run(run_args: RunArgs) -> anyhow::Result<()> {
    // Check if file exists only if provided via -f
    if let Some(file_path) = &run_args.file {
        check_file_path(file_path)?;
    }

    let file_path = run_args.file.unwrap_or(DEFAULT_PARTY_CONF.to_string());

    let commands = if Path::new(&file_path).exists() {
        let parser = CommandParser {
            path: file_path.to_string(),
        };
        let tasks = parser.parse()?;
        party_command::convert_toml_tasks(tasks.tasks)
    } else {
        make_default_commands()
    };

    match run_args.index {
        Some(index) => run_single_task(commands, index),
        None => run_all(commands).await,
    }
}

fn run_single_task(commands: Vec<PartyCommand>, index: usize) -> anyhow::Result<()> {
    println!("Staring party run 🏇🏇🏇");

    let Some(command) = commands.get(index - 1) else {
        bail!(
            "Index {} out of range. Only {} tasks are available",
            index,
            commands.len()
        )
    };

    println!("Running a single task: {}", command);
    runner::run_single_command(command)?;
    println!("✅ Party run complete! ✅");

    Ok(())
}

async fn run_all(commands: Vec<PartyCommand>) -> anyhow::Result<()> {
    println!("Staring party run 🏇🏇🏇");

    let no_commands = commands.len();

    let batches = schdeuler::schedule_commands(commands);

    if batches.len() == no_commands {
        runner::run_sync_commands(batches, no_commands)?;
    } else {
        runner::run_async_commands(batches, no_commands).await?;
    }

    println!("✅ Party run complete! ✅");

    Ok(())
}
