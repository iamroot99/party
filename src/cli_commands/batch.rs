//! `party batch`

use std::path::Path;

use crate::{
    cli::BatchArgs,
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    schdeuler,
    util::{check_file_path, make_counter_blue, make_message_bright_green, DEFAULT_PARTY_CONF},
};

/// Implementation of `party batch`
pub fn batch(batch_args: BatchArgs) -> anyhow::Result<()> {
    // Check if file exists only if provided via -f
    if let Some(file_path) = &batch_args.file {
        check_file_path(file_path)?;
    }

    let file_path = batch_args.file.unwrap_or(DEFAULT_PARTY_CONF.to_string());
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

    println!(
        "{} tasks will be run in {} batches. All tasks in a batch are run in parallel.",
        no_commands,
        batches.len()
    );

    for (i, batch) in batches.iter().enumerate() {
        println!(
            "Batch {} with {}:",
            make_counter_blue(i + 1, batches.len()),
            make_message_bright_green(&format!("{} commands", batch.len()))
        );
        for task in batch {
            println!("  - {}", task);
        }
    }

    Ok(())
}
