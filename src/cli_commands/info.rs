//! `party info`
use std::path::Path;

use crate::{
    cli::InfoArgs,
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    util::{check_file_path, make_counter_blue, make_message_bright_green, DEFAULT_PARTY_CONF},
};

/// Implementation of `party info`
pub fn info(info_args: InfoArgs) -> anyhow::Result<()> {
    // Check if file exists only if provided via -f
    if let Some(file_path) = &info_args.file {
        check_file_path(file_path)?;
    }

    let file_path = info_args.file.unwrap_or(DEFAULT_PARTY_CONF.to_string());
    let commands;
    let config_exists;

    if Path::new(&file_path).exists() {
        let parser = CommandParser {
            path: file_path.to_string(),
        };
        let tasks = parser.parse()?;
        commands = party_command::convert_toml_tasks(tasks.tasks);
        config_exists = true;
    } else {
        commands = make_default_commands();
        config_exists = false;
    };
    let no_commands = commands.len();

    if config_exists {
        print!(
            "{} has a total of {} tasks.",
            make_message_bright_green(&file_path),
            no_commands
        );
    } else {
        print!(
            "Default configuration has a total of {} tasks.",
            no_commands
        );
    }
    println!(
        " Tasks marked with {} are run in parallel:\n",
        make_message_bright_green("[P]")
    );

    for (i, command) in commands.iter().enumerate() {
        let prefix = if command.is_parallel { "[P]" } else { "[ ]" };

        println!(
            "{}{}: {}",
            make_message_bright_green(prefix),
            make_counter_blue(i + 1, no_commands),
            command
        );
    }

    Ok(())
}
