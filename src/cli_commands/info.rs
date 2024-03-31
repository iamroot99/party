//! `party info`
use std::path::Path;

use crate::{
    cli::InfoArgs,
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    util::{make_counter_blue, make_message_bright_green},
};

/// Implementation of `party info`
pub fn info(info_args: InfoArgs) -> anyhow::Result<()> {
    let file_path = info_args.file;
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

    println!("Configuration has a total of {} tasks:", no_commands);
    for (i, command) in commands.iter().enumerate() {
        let prefix = if command.is_parallel { "[P]" } else { "[ ]" };

        println!(
            "{}{}: {}",
            make_message_bright_green(prefix),
            make_counter_blue(i, no_commands),
            command
        );
    }

    Ok(())
}
