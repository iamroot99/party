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
    /// party run
    Run(RunArgs),

    /// Display configuration information
    Info(InfoArgs),

    /// Display scheduled batches information
    Batch(BatchArgs),
}

/// Arguments for party run
#[derive(Args)]
pub struct RunArgs {
    /// Party configuration file. If missing, default tasks are used
    #[arg(short, long)]
    pub file: Option<String>,

    /// Index of task to run from the configuration file
    #[arg(short, long)]
    pub index: Option<usize>,
}

/// Arguments for party info
#[derive(Args)]
pub struct InfoArgs {
    /// Party configuration file to describe. If missing, default tasks are used
    #[arg(short, long)]
    pub file: Option<String>,
}

/// Arguments for party batch
#[derive(Args)]
pub struct BatchArgs {
    /// Party configuration file to describe. If missing, default tasks are used
    #[arg(short, long)]
    pub file: Option<String>,
}
