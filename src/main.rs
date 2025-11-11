use clap::{ArgGroup, Parser, Subcommand};
mod battery;
mod cpu;
mod disk;
mod mem;
mod util;

#[derive(Subcommand, Debug)]
enum Commands {
    /// CPU metrics
    Cpu {
        /// Measurement interval (0 = instantaneous, >0 = seconds, blocking)
        #[arg(short = 'i', long = "interval", default_value_t = 1u64)]
        interval: u64,
        /// Per-CPU output (comma-separated floats, no %)
        #[arg(long = "percpu", default_value_t = false)]
        percpu: bool,
    },

    /// Memory metrics
    Mem {
        /// Show used/total (e.g., 7.23GB/31.26GB)
        #[arg(short = 't', long = "total", default_value_t = false)]
        total: bool,
    },

    /// Disk metrics
    #[command(group(
        ArgGroup::new("disk_view")
            .args(["total", "free"])
            .multiple(false)
    ))]
    Disk {
        /// Target path or mount
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
        /// Show used/total humanized
        #[arg(short = 't', long = "total", default_value_t = false)]
        total: bool,
        /// Show free humanized
        #[arg(short = 'f', long = "free", default_value_t = false)]
        free: bool,
    },

    /// Battery metrics
    #[command(
        group(
            ArgGroup::new("battery_type")
                .args(["percent", "time"])
                .multiple(false)
        ),
        group(
            ArgGroup::new("battery_tone")
                .args(["long", "fun"])
                .multiple(false)
        )
    )]
    Battery {
        /// Percent output or default (Charging or <rounded>%)
        #[arg(short = 'p', long = "percent", default_value_t = false)]
        percent: bool,
        /// Time remaining (H:MM:SS) or "Charging"
        #[arg(short = 't', long = "time", default_value_t = false)]
        time: bool,
        /// Long text tone
        #[arg(short = 'l', long = "long", default_value_t = false)]
        long: bool,
        /// Fun text tone
        #[arg(short = 'f', long = "fun", default_value_t = false)]
        fun: bool,
        /// Compact glyph output
        #[arg(short = 'c', long = "compact", default_value_t = false)]
        compact: bool,
    },
}

#[derive(Parser, Debug)]
#[command(
    name = "tmux-sysmon",
    author,
    version,
    about = "tmux status metrics (cpu, mem, disk, battery)",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Cpu { interval, percpu } => cpu::run(interval, percpu),
        Commands::Mem { total } => mem::run(total),
        Commands::Disk { path, total, free } => disk::run(path, total, free),
        Commands::Battery {
            percent,
            time,
            long,
            fun,
            compact,
        } => battery::run(percent, time, long, fun, compact),
    }
}
