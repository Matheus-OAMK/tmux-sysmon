# tmux-sysmon

A tmux plugin for monitoring system resources (CPU, memory, disk, battery) written in Rust.

## Features

- **CPU**: Global or per-core usage monitoring
- **Memory**: RAM usage with percentage or used/total display
- **Disk**: Disk usage for any mount point with humanized sizes
- **Battery**: Battery status with multiple display formats

## Installation

### Requirements

- Rust 1.75 or later
- tmux

### Building from source

```bash
cargo build --release
```

The binary will be available at `target/release/tmux-sysmon`.

## Usage

### Command Line

```bash
# CPU usage
./target/release/tmux-sysmon cpu                    # → 12.3%
./target/release/tmux-sysmon cpu --interval 0       # → instantaneous
./target/release/tmux-sysmon cpu --percpu           # → per-core list

# Memory usage
./target/release/tmux-sysmon mem                    # → 52.7%
./target/release/tmux-sysmon mem --total            # → 7.23GB/31.26GB

# Disk usage
./target/release/tmux-sysmon disk                   # → 31.6%
./target/release/tmux-sysmon disk --free            # → 55.7G
./target/release/tmux-sysmon disk --total           # → 12.3G/55.7G
./target/release/tmux-sysmon disk --path /home      # → usage for /home

# Battery status
./target/release/tmux-sysmon battery                # → Charging or 85%
./target/release/tmux-sysmon battery --time         # → 2:34:12
./target/release/tmux-sysmon battery --long         # → "More than 3/4 full"
./target/release/tmux-sysmon battery --compact      # → █ (single glyph)
```

### Tmux Integration

The plugin provides a script that automatically replaces placeholders in your tmux status bar with live metrics.

#### Setup

1. Build the release binary:
   ```bash
   cargo build --release
   ```

2. Add placeholders to your tmux status bar:
   ```bash
   tmux set -g status-right "CPU: #{cpu} | MEM: #{mem} | DISK: #{disk} | BAT: #{battery}"
   ```

3. Run the integration script:
   ```bash
   tmux run-shell /path/to/tmux-cpu-mem-monitor/tmux-sysmon.tmux
   ```

#### Making it Permanent

Add to your `.tmux.conf`:

```bash
# Add your status bar configuration with placeholders
set -g status-right "CPU: #{cpu} | MEM: #{mem} | DISK: #{disk}"

# Run the integration script
run-shell /path/to/tmux-cpu-mem-monitor/tmux-sysmon.tmux
```

Then reload your tmux config:
```bash
tmux source-file ~/.tmux.conf
```

#### Supported Placeholders

- `#{cpu}` - CPU usage percentage
- `#{mem}` - Memory usage percentage
- `#{disk}` - Disk usage percentage
- `#{battery}` - Battery status

You can pass flags through placeholders:
- `#{cpu --interval 0}` - Instantaneous CPU reading
- `#{mem --total}` - Show used/total memory
- `#{disk --free}` - Show free disk space
- `#{battery --compact}` - Battery as single glyph

## Platform Support

- **Linux**: Full functionality
- **macOS**: CPU, memory, disk, and battery supported
- **Windows**: CPU, memory, and disk supported; battery via `battery` crate

## Notes

- CPU `--interval > 0` is blocking; use `--interval 0` for non-blocking updates in status bars
- Systems without a battery will show `N/A`
- Default disk path is the root filesystem unless specified with `--path`

## License

MIT
