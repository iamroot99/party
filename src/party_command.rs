//! Core party command.
use core::fmt;

use crate::parser::toml_command::Task;

/// Struct holding a structured PartyCommand containing a command,
/// its arguments as an array and wether it can be parallelised.
pub struct PartyCommand {
    /// Command to run
    pub command: String,

    /// Signals if command can be paralelised
    pub is_parallel: bool,
}

impl PartyCommand {
    /// Create a new PartyCommand
    pub fn new(command: String, is_parallel: bool) -> Self {
        Self {
            command,
            is_parallel,
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
    let cargo_fmt = PartyCommand::new("cargo fmt".to_string(), false);

    let cargo_clippy = PartyCommand::new("cargo clippy -- -Dwarnings".to_string(), false);

    let cargo_test = PartyCommand::new("cargo test".to_string(), false);

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
        let cmd = PartyCommand::new("cargo clippy -- -Dwarnings".to_string(), false);

        // WHEN
        let cmd_string = format!("{}", cmd);

        // THEN
        assert_eq!(cmd_string, "cargo clippy -- -Dwarnings");
    }
}
