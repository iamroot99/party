//! Asynchronous task runner
use anyhow::Context;

use crate::{
    runner::{
        command_handler::handle_single_command, output_util::make_counter_blue,
        run_report::RunReport,
    },
    schdeuler::CommandBatch,
};

use super::output_util::print_status_report;

/// Main driver function running all the party commands in batches asynchronously.
pub async fn run_async_commands(
    batches: Vec<CommandBatch>,
    no_commands: usize,
) -> anyhow::Result<()> {
    let mut reports: Vec<RunReport> = vec![];
    let mut failed = 0;

    let mut curr: usize = 0;

    for batch in batches {
        let mut handles = vec![];

        for command in batch {
            curr += 1;

            handles.push(tokio::spawn(async move {
                handle_single_command(make_counter_blue(curr, no_commands), &command)
            }));
        }

        for handle in handles {
            let report = handle.await.context("Failed to join concurrent task")??;
            if !report.success {
                failed += 1;
            }

            reports.push(report);
        }
    }

    print_status_report(failed, no_commands, reports);

    Ok(())
}
