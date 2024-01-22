//! Command parser handling user-provided TOML commmands.
use std::fs;

use anyhow::{bail, Context};

use super::toml_command::Tasks;

/// Command parser that takes a TOML path and parses its contents.
pub struct CommandParser {
    /// File path
    pub path: String,
}

impl CommandParser {
    /// Parse the contents of the inner file and return the parsed set of tasks.
    /// Returns an error if a command is not well formed (empty).
    pub fn parse(&self) -> anyhow::Result<Tasks> {
        let contents = fs::read_to_string(self.path.clone())
            .context(format!("Failed to read from {}", self.path))?;

        let tasks: Tasks = toml::from_str(&contents).context("Invalid TOML data")?;

        for task in &tasks.tasks {
            if task.command.is_empty() {
                bail!("Inalid command");
            }
        }

        Ok(tasks)
    }
}
