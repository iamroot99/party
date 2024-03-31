//! CLI argument handler
use clap::{arg, command, Args, Parser, Subcommand};

/// Simple local development automator
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    /// Client command
    #[command(subcommand)]
    pub command: CliCommands,
}

/// Possible client commands
#[derive(Subcommand)]
pub enum CliCommands {
    /// Run cargo party
    Run(RunArgs),
}

/// Arguments for cargo party run
#[derive(Args)]
pub struct RunArgs {
    /// Party configuration file. If missing, default tasks are used
    #[arg(short, long, default_value = "./party.toml")]
    pub file: String,
}
