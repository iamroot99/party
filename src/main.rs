use std::path::Path;

use cargo_party::{
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    runner, schdeuler,
};
use clap::{command, Parser};

/// Simple local development automator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Party configuration file. If missing, default tasks are used
    #[arg(short, long, default_value = "./party.toml")]
    file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let file_path = args.file;

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
