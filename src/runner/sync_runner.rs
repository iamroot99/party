//! Single-threaded runner.
use crate::{
    party_command::PartyCommand,
    runner::{command_handler::handle_single_command, run_report::RunReport},
    schdeuler::CommandBatch,
    util::make_counter_blue,
};

use super::run_report::print_status_report;

/// Main driver function running all the party commands synchronously.
pub fn run_sync_commands(batches: Vec<CommandBatch>, no_commands: usize) -> anyhow::Result<()> {
    let mut reports: Vec<RunReport> = vec![];
    let mut failed = 0;

    let mut curr: usize = 0;

    for batch in batches {
        assert!(batch.len() == 1);

        let command = &batch[0];
        curr += 1;

        let report = handle_single_command(make_counter_blue(curr, no_commands), command)?;
        if !report.success {
            failed += 1;
        }

        reports.push(report);
    }

    print_status_report(failed, no_commands, reports);

    Ok(())
}

/// Run a single command
pub fn run_single_command(command: &PartyCommand) -> anyhow::Result<()> {
    let report = handle_single_command(make_counter_blue(1, 1), command)?;

    let failed = if !report.success { 1 } else { 0 };

    print_status_report(failed, 1, vec![report]);
    Ok(())
}
