//! Structs used by the parser to parse the TOML components.
use serde::{Deserialize, Serialize};

use crate::party_command::PartyCommand;

/// Single TOML task
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// Command components
    pub command: String,

    /// Signals if command can be paralelised
    pub parallel: Option<bool>,
}

/// Top-level struct holding all TOML tasks
#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    /// Parsed tasks
    pub tasks: Vec<Task>,
}

impl From<PartyCommand> for Task {
    fn from(value: PartyCommand) -> Self {
        // Set to None if the task is not parallel
        let parallel = value.is_parallel.then_some(true);

        Self {
            command: value.command,
            parallel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tasks;

    #[test]
    fn test_parse_toml() {
        // GIVEN
        let toml = "
        [[tasks]]
        command = \"cargo fmt\"
        parallel = false

        [[tasks]]
        command = \"cargo clippy -- -Dwarnings\"
        parallel = true

        [[tasks]]
        command = \"cargo test\"
        ";

        // WHEN
        let tasks: Result<Tasks, toml::de::Error> = toml::from_str(&toml);

        // THEN
        assert!(tasks.is_ok());
        let tasks = tasks.unwrap();

        assert_eq!(tasks.tasks.len(), 3);

        assert_eq!(tasks.tasks[0].command, "cargo fmt");
        assert!(tasks.tasks[0].parallel.is_some_and(|x| !x));

        assert_eq!(tasks.tasks[1].command, "cargo clippy -- -Dwarnings");
        assert!(tasks.tasks[1].parallel.is_some_and(|x| x));

        assert_eq!(tasks.tasks[2].command, "cargo test");
        assert!(tasks.tasks[2].parallel.is_none());
    }
}
