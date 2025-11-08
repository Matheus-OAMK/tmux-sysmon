pub fn fmt_percent_1dp(value: f64) -> String {
    // Round to one decimal place and append %
    format!("{:.1}%", value)
}

pub fn fmt_gb_two_decimals(bytes: u64) -> String {
    // Convert bytes to GiB but label as GB (to match legacy examples)
    let gib = bytes as f64 / 1024f64.powi(3);
    format!("{:.2}GB", gib)
}

pub fn humanize_bytes_short(bytes: u64) -> String {
    // Humanize bytes into short units K/M/G/T with one decimal, no 'B' suffix (e.g., "55.7G")
    const UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];
    if bytes < 1024 {
        return "0.0K".to_string();
    }
    let mut value = bytes as f64;
    let mut idx = 0usize;
    while value >= 1024.0 && idx < UNITS.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }
    let unit = UNITS[idx];
    if unit == "B" {
        format!("{:.1}B", value)
    } else {
        format!("{:.1}{}", value, unit)
    }
}

pub fn fmt_time_hms(total_seconds: u64) -> String {
    // Format as H:MM:SS
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_1dp() {
        assert_eq!(fmt_percent_1dp(12.34), "12.3%");
    }

    #[test]
    fn gb_two_decimals() {
        let gib = 31.26_f64;
        let bytes = (gib * 1024f64.powi(3)) as u64;
        assert!(fmt_gb_two_decimals(bytes).starts_with("31.26GB"));
    }

    #[test]
    fn humanize_short() {
        assert_eq!(
            humanize_bytes_short(1024 * 1024 * 1024 + 600 * 1024 * 1024),
            "1.6G"
        );
    }

    #[test]
    fn time_hms() {
        assert_eq!(fmt_time_hms(8100 + 5), "2:15:05");
    }
}
