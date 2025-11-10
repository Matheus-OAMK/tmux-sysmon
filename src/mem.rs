use crate::util::format::{fmt_gb_two_decimals, fmt_percent_1dp};

pub fn run(total: bool) {
    // Use sysinfo memory API: compute used = total - available for a user-friendly percent.
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();

    let total_bytes = sys.total_memory();
    let avail_bytes = sys.available_memory();
    let used_bytes = total_bytes.saturating_sub(avail_bytes);

    if total {
        let used = fmt_gb_two_decimals(used_bytes);
        let total = fmt_gb_two_decimals(total_bytes);
        println!("{}/{}", used, total);
    } else {
        let pct = if total_bytes > 0 {
            (used_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };
        println!("{}", fmt_percent_1dp(pct));
    }
}
