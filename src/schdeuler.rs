//! Batch command scheduler
use crate::party_command::PartyCommand;

/// Alias for a vector of Party commands
pub type CommandBatch = Vec<PartyCommand>;

/// Schedule a vector of Party commands into batches.
/// If a series of commands have the parallel flag set to `true`
/// they are batched together and will be run in parallel.
/// Commands that have the parallel flag set to `false` will be put into
/// their own batches of size 1.
pub fn schedule_commands(commands: Vec<PartyCommand>) -> Vec<CommandBatch> {
    let mut batches = vec![];

    let mut batch = vec![];
    for command in commands {
        if command.is_parallel {
            batch.push(command);
        } else {
            // If there is at least a command in the current batch push it
            if !batch.is_empty() {
                batches.push(batch);
            }

            // Push the current command alone
            batches.push(vec![command]);

            // Prepare the new batch
            batch = vec![];
        }
    }

    // Push the last batch if not empty
    if !batch.is_empty() {
        batches.push(batch);
    }

    batches
}

#[cfg(test)]
pub mod test {
    use crate::party_command::PartyCommand;

    use super::schedule_commands;

    fn parallel_command() -> PartyCommand {
        PartyCommand::new("".into(), vec![], true)
    }

    fn sequential_command() -> PartyCommand {
        PartyCommand::new("".into(), vec![], false)
    }

    #[test]
    fn test_scheduler_ending_with_sequential() {
        // GIVEN
        let commands = vec![
            parallel_command(),
            parallel_command(),
            sequential_command(),
            parallel_command(),
            parallel_command(),
            parallel_command(),
            sequential_command(),
            sequential_command(),
        ];

        // WHEN
        let batches = schedule_commands(commands);

        // THEN
        assert_eq!(batches.len(), 5);
        assert_eq!(batches[0].len(), 2);
        assert_eq!(batches[1].len(), 1);
        assert_eq!(batches[2].len(), 3);
        assert_eq!(batches[3].len(), 1);
        assert_eq!(batches[4].len(), 1);
    }

    #[test]
    fn test_scheduler_ending_with_parallel() {
        // GIVEN
        let commands = vec![
            parallel_command(),
            parallel_command(),
            sequential_command(),
            parallel_command(),
            parallel_command(),
            parallel_command(),
        ];

        // WHEN
        let batches = schedule_commands(commands);

        // THEN
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].len(), 2);
        assert_eq!(batches[1].len(), 1);
        assert_eq!(batches[2].len(), 3);
    }
}
