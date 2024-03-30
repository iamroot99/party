//! Single-threaded runner.
use crate::{
    runner::{
        command_handler::handle_single_command, output_util::make_counter_blue,
        run_report::RunReport,
    },
    schdeuler::CommandBatch,
};

use super::output_util::print_status_report;

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
