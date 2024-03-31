use std::path::Path;

use anyhow::Ok;
use cargo_party::{
    cli::{CliArgs, CliCommands, InfoArgs, RunArgs},
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    runner, schdeuler,
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CliArgs::parse();
    match cli.command {
        CliCommands::Run(run_args) => run(run_args).await,
        CliCommands::Info(info_args) => info(info_args),
        CliCommands::Batch => batch(),
    }
}

fn batch() -> anyhow::Result<()> {
    println!("TODO!");
    Ok(())
}

fn info(info_args: InfoArgs) -> anyhow::Result<()> {
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

    println!("Parsed a total of {} tasks:", no_commands);
    for (i, command) in commands.iter().enumerate() {
        let prefix = if command.is_parallel { "[P]" } else { "[ ]" };

        println!("{}[{}/{}]: {}", prefix, i, no_commands, command);
    }

    Ok(())
}

async fn run(run_args: RunArgs) -> anyhow::Result<()> {
    let file_path = run_args.file;

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
