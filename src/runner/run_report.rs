pub struct RunReport {
    pub message: String,
    pub success: bool,
}

impl RunReport {
    pub fn new_success(message: String) -> Self {
        Self {
            message,
            success: true,
        }
    }

    pub fn new_failed(message: String) -> Self {
        Self {
            message,
            success: false,
        }
    }
}

/// Display the status report for the current run
pub fn print_status_report(failed: usize, total: usize, reports: Vec<RunReport>) {
    if failed != 0 {
        println!("\nparty run report - {}/{} failed tasks:", failed, total);
    } else {
        println!("\nparty run report - all tasks passed:");
    }
    for report in reports {
        if report.success {
            println!("{}", report.message);
        } else {
            eprintln!("{}", report.message);
        }
    }
    println!();
}
