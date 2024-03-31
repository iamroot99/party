//! `party run`
use std::path::Path;

use crate::{
    cli::RunArgs,
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
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
    let no_commands = commands.len();

    let batches = schdeuler::schedule_commands(commands);

    println!("Staring cargo party 🏇🏇🏇");

    if batches.len() == no_commands {
        runner::run_sync_commands(batches, no_commands)?;
    } else {
        runner::run_async_commands(batches, no_commands).await?;
    }

    println!("✅ Cargo party complete! ✅");

    Ok(())
}
