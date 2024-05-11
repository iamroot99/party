use clap::Parser;
use party::{
    cli::{CliArgs, CliCommands},
    cli_commands,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CliArgs::parse();

    match cli.command {
        CliCommands::Init(init_args) => cli_commands::init(init_args),
        CliCommands::Run(run_args) => cli_commands::run(run_args).await,
        CliCommands::Info(info_args) => cli_commands::info(info_args),
        CliCommands::Batch(batch_args) => cli_commands::batch(batch_args),
    }
}
