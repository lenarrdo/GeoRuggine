use std::{fs::OpenOptions, io::Write, time::Duration};

use chrono::Local;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

pub fn start(interval: Duration, output_path: &str) {
    let output_path = output_path.to_string();

    std::thread::spawn(move || {
        let mut sys = System::new();
        let pid = Pid::from_u32(std::process::id());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_path)
            .expect("Failed to open CPU log file");

        loop {
            // Refresh process info for the current process
            sys.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[pid]),
                true,
                ProcessRefreshKind::nothing().with_cpu(),
            );

            if let Some(process) = sys.process(pid) {
                let cpu_usage = process.cpu_usage();
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

                let _ = writeln!(file, "[{}] CPU Usage: {:.2}%", timestamp, cpu_usage);
                let _ = file.flush();
            }

            std::thread::sleep(interval);
        }
    });
}
