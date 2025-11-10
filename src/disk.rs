use crate::util::format::{fmt_percent_1dp, humanize_bytes_short};
use crate::util::platform::pick_path;
use std::path::Path;
use sysinfo::Disks;

pub fn run(path: Option<String>, total: bool, free: bool) {
    let target = pick_path(path);
    let target_path = Path::new(&target);

    // Refresh disks and pick the best match for the target path (longest mount prefix)
    let disks = Disks::new_with_refreshed_list();
    let mut best: Option<(&sysinfo::Disk, usize)> = None;

    for disk in &disks {
        let mp = disk.mount_point();
        // Use string length of mount point as tie-breaker for "longest prefix"
        if target_path.starts_with(mp) {
            let score = mp.as_os_str().len();
            match best {
                Some((_, cur)) if score <= cur => {}
                _ => best = Some((disk, score)),
            }
        }
    }

    let disk = match best {
        Some((d, _)) => d,
        None => {
            // Fallback: if no matching mount, pick the first disk if any
            if let Some(d) = disks.iter().next() {
                d
            } else {
                println!("N/A");
                return;
            }
        }
    };

    let total_bytes = disk.total_space();
    if total_bytes == 0 {
        println!("N/A");
        return;
    }
    let avail_bytes = disk.available_space();
    let used_bytes = total_bytes.saturating_sub(avail_bytes);

    if free {
        println!("{}", humanize_bytes_short(avail_bytes));
        return;
    }

    if total {
        let used_h = humanize_bytes_short(used_bytes);
        let total_h = humanize_bytes_short(total_bytes);
        println!("{}/{}", used_h, total_h);
        return;
    }

    // Default: percentage used
    let pct = (used_bytes as f64 / total_bytes as f64) * 100.0;
    println!("{}", fmt_percent_1dp(pct));
}
