//! Structs used by the parser to parse the TOML components.
use serde::Deserialize;

/// Single TOML task
#[derive(Deserialize, Debug)]
pub struct Task {
    /// Command components
    pub command: Vec<String>,

    /// Signals if command can be paralelised
    pub parallel: Option<bool>,
}

/// Top-level struct holding all TOML tasks
#[derive(Deserialize, Debug)]
pub struct Tasks {
    /// Parsed tasks
    pub tasks: Vec<Task>,
}

#[cfg(test)]
mod tests {
    use super::Tasks;

    #[test]
    fn test_parse_toml() {
        // GIVEN
        let toml = "
        [[tasks]]
        command = [\"cargo\", \"fmt\"]
        parallel = false

        [[tasks]]
        command = [\"cargo\", \"clippy\", \"--\", \"-Dwarnings\"]
        parallel = true

        [[tasks]]
        command = [\"cargo\", \"test\"]
        ";

        // WHEN
        let tasks: Result<Tasks, toml::de::Error> = toml::from_str(&toml);

        // THEN
        assert!(tasks.is_ok());
        let tasks = tasks.unwrap();

        assert_eq!(tasks.tasks.len(), 3);

        assert_eq!(tasks.tasks[0].command.len(), 2);
        assert!(tasks.tasks[0].parallel.is_some_and(|x| !x));

        assert_eq!(tasks.tasks[1].command.len(), 4);
        assert!(tasks.tasks[1].parallel.is_some_and(|x| x));

        assert_eq!(tasks.tasks[2].command.len(), 2);
        assert!(tasks.tasks[2].parallel.is_none());
    }
}
