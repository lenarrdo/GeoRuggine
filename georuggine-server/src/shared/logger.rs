use chrono::Local;
use cpu_time::ProcessTime;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};

pub fn start(interval: Duration, output_path: &str) {
    let output_path = output_path.to_string();

    std::thread::spawn(move || {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_path)
            .expect("Failed to open CPU log file");

        let process_start_cpu = ProcessTime::now();

        let mut last_cpu_time = process_start_cpu;
        let mut last_real_time = Instant::now();

        loop {
            std::thread::sleep(interval);

            let current_cpu_time = ProcessTime::now();
            let current_real_time = Instant::now();

            let cpu_delta = current_cpu_time.duration_since(last_cpu_time);
            let real_delta = current_real_time.duration_since(last_real_time);

            let cpu_usage = (cpu_delta.as_secs_f64() / real_delta.as_secs_f64()) * 100.0;

            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

            let _ = writeln!(
                file,
                "[{timestamp}] CPU Usage: {cpu_usage:.2}% | CPU Time: {:.2?}",
                process_start_cpu.elapsed()
            );
            let _ = file.flush();

            last_cpu_time = current_cpu_time;
            last_real_time = current_real_time;
        }
    });
}
