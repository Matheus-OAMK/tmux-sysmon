use crate::util::format::fmt_percent_1dp;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub fn run(interval: u64, percpu: bool) {
    // Initialize with CPU refresh kind to keep it lightweight
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    sys.refresh_cpu(); // baseline

    if percpu {
        if interval == 0 {
            // Retry small sampling windows up to ~250ms total to accumulate deltas.
            let deadline = Instant::now() + Duration::from_millis(250);
            let mut vals: Vec<f32>;

            loop {
                thread::sleep(Duration::from_millis(25));
                sys.refresh_cpu();
                vals = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
                if vals.iter().any(|v| *v > 0.0) || Instant::now() >= deadline {
                    break;
                }
            }

            let parts: Vec<String> = vals.iter().map(|v| format!("{:.1}", v)).collect();
            println!("{}", parts.join(", "));
        } else {
            thread::sleep(Duration::from_secs(interval));
            sys.refresh_cpu();
            let parts: Vec<String> = sys
                .cpus()
                .iter()
                .map(|c| format!("{:.1}", c.cpu_usage()))
                .collect();
            println!("{}", parts.join(", "));
        }
    } else {
        if interval == 0 {
            // Retry small windows up to ~250ms
            let deadline = Instant::now() + Duration::from_millis(250);
            loop {
                thread::sleep(Duration::from_millis(25));
                sys.refresh_cpu();
                let usage = sys.global_cpu_info().cpu_usage() as f64;
                if usage > 0.0 || Instant::now() >= deadline {
                    println!("{}", fmt_percent_1dp(usage));
                    break;
                }
            }
        } else {
            thread::sleep(Duration::from_secs(interval));
            sys.refresh_cpu();
            let usage = sys.global_cpu_info().cpu_usage() as f64;
            println!("{}", fmt_percent_1dp(usage));
        }
    }
}
