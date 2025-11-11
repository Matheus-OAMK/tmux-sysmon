use crate::util::format::fmt_time_hms;
use battery::{Manager, State};

pub fn run(_percent: bool, time: bool, long: bool, fun: bool, compact: bool) {
    // Priority: compact > (long|fun) > time > percent/default
    let mgr = match Manager::new() {
        Ok(m) => m,
        Err(_) => {
            println!("N/A");
            return;
        }
    };

    let mut batteries = match mgr.batteries() {
        Ok(iter) => iter,
        Err(_) => {
            println!("N/A");
            return;
        }
    };

    let bat = match batteries.next() {
        Some(Ok(b)) => b,
        _ => {
            println!("N/A");
            return;
        }
    };

    let is_charging = matches!(bat.state(), State::Charging);
    let soc_ratio = bat.state_of_charge();
    let pct_f = (soc_ratio.value * 100.0).clamp(0.0, 100.0);
    let pct_rounded = pct_f.round() as i32;

    // Compact glyph mode
    if compact {
        if is_charging {
            // U+2593 when charging (single glyph, one line)
            println!("\u{2593}");
        } else {
            // Map 0..100 -> 0..7 then U+2581..U+2588
            let mut level = ((pct_rounded.max(0) as u32).min(100) * 8) / 100;
            if level > 7 {
                level = 7;
            }
            let ch = char::from_u32(0x2581 + level).unwrap_or('\u{2581}');
            println!("{}", ch);
        }
        return;
    }

    // Long/fun textual ranges
    if long || fun {
        if is_charging {
            println!("Charging");
            return;
        }
        println!("{}", battery_phrase(pct_rounded));
        return;
    }

    // Time output
    if time {
        if is_charging {
            println!("Charging");
            return;
        }
        match bat.time_to_empty() {
            Some(dur) => {
                let secs = dur.get::<battery::units::time::second>() as u64;
                println!("{}", fmt_time_hms(secs));
            }
            None => println!("N/A"),
        }
        return;
    }

    // Percent/default output
    if is_charging {
        println!("Charging");
        return;
    }
    println!("{}%", pct_rounded);
}

fn battery_phrase(pct_rounded: i32) -> &'static str {
    match pct_rounded {
        100 => "Fully charged",
        95..=99 => "Almost full",
        74..=94 => "More than 3/4 full",
        50..=73 => "More than half full",
        26..=49 => "Less than half full",
        6..=25 => "Battery is running low",
        2..=5 => "Battery is almost empty",
        1 => "I'm dying over here",
        _ => "Out of battery",
    }
}
