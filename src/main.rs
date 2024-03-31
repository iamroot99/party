use cargo_party::{
    cli::{CliArgs, CliCommands},
    cli_commands,
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = CliArgs::parse();

    match cli.command {
        CliCommands::Run(run_args) => cli_commands::run(run_args).await,
        CliCommands::Info(info_args) => cli_commands::info(info_args),
        CliCommands::Batch => cli_commands::batch(),
    }
}
