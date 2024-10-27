//! Core party command.
use core::fmt;

use crate::{parser::toml_command::Task, util::OptionEnv};

/// Struct holding a structured PartyCommand containing a command,
/// wether it can be parallelised and any environment variables it requires.
#[derive(Debug)]
pub struct PartyCommand {
    /// Command to run
    pub command: String,

    /// Environment variables for the current comand
    pub env: OptionEnv,

    /// Signals if command can be paralelised
    pub is_parallel: bool,
}

impl PartyCommand {
    /// Create a new PartyCommand
    pub fn new(command: String, is_parallel: bool, env: OptionEnv) -> Self {
        Self {
            command,
            is_parallel,
            env,
        }
    }
}

impl From<Task> for PartyCommand {
    fn from(task: Task) -> Self {
        assert!(!task.command.is_empty());

        // A task is not parallel by default
        let is_parallel = task.parallel.unwrap_or(false);

        Self {
            command: task.command,
            env: task.env,
            is_parallel,
        }
    }
}

impl fmt::Display for PartyCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

/// Create the default party commands.
/// By default party runs the following commands sequentially:
/// - cargo fmt
/// - cargo clippy -- -Dwarnings
/// - cargo test
pub fn make_default_commands() -> Vec<PartyCommand> {
    let cargo_fmt = PartyCommand::new("cargo fmt".to_string(), false, None);

    let cargo_clippy = PartyCommand::new("cargo clippy -- -Dwarnings".to_string(), false, None);

    let cargo_test = PartyCommand::new("cargo test".to_string(), false, None);

    vec![cargo_fmt, cargo_clippy, cargo_test]
}

/// Convert a vector of TOML tasks into structured Party commands
pub fn convert_toml_tasks(tasks: Vec<Task>) -> Vec<PartyCommand> {
    tasks.into_iter().map(PartyCommand::from).collect()
}

#[cfg(test)]
mod test {
    use crate::party_command::PartyCommand;

    #[test]
    fn test_display() {
        // GIVEN
        let cmd = PartyCommand::new("cargo clippy -- -Dwarnings".to_string(), false, None);

        // WHEN
        let cmd_string = format!("{}", cmd);

        // THEN
        assert_eq!(cmd_string, "cargo clippy -- -Dwarnings");
    }
}
