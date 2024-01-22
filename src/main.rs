use std::path::Path;

use cargo_party::{
    parser::command_parser::CommandParser,
    party_command::{self, make_default_commands},
    runner, schdeuler,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_path = "./party.toml";

    let commands = match Path::new(file_path).exists() {
        true => {
            let parser = CommandParser {
                path: file_path.to_string(),
            };
            let tasks = parser.parse()?;
            party_command::convert_toml_tasks(tasks.tasks)
        }
        false => make_default_commands(),
    };
    let no_commands = commands.len();

    let batches = schdeuler::schedule_commands(commands);

    println!("Staring cargo party!");
    runner::run_commands(batches, no_commands).await?;
    println!("Cargo party complete");

    Ok(())
}
