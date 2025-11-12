#!/usr/bin/env bash

CURRENT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Find the tmux-sysmon binary
# Priority: 1) release binary, 2) try to build release, 3) in PATH
TMUX_SYSMON_BIN=""

if [ -x "$CURRENT_DIR/target/release/tmux-sysmon" ]; then
    TMUX_SYSMON_BIN="$CURRENT_DIR/target/release/tmux-sysmon"
elif [ -f "$CURRENT_DIR/Cargo.toml" ]; then
    # Try to build release binary
    tmux display-message "tmux-sysmon: Building binary..."
    cd "$CURRENT_DIR"
    if cargo build --release --quiet 2>/dev/null; then
        TMUX_SYSMON_BIN="$CURRENT_DIR/target/release/tmux-sysmon"
    fi
fi

# Fallback to PATH
if [ -z "$TMUX_SYSMON_BIN" ] && command -v tmux-sysmon >/dev/null 2>&1; then
    TMUX_SYSMON_BIN="tmux-sysmon"
fi

# If binary still not found, display error and exit
if [ -z "$TMUX_SYSMON_BIN" ]; then
    tmux display-message "tmux-sysmon: Binary not found. Please run 'cargo build --release'."
    exit 1
fi

# Function to replace placeholders in tmux options
# Usage: replace_placeholders "option_name" "option_value"
replace_placeholders() {
    local option_name="$1"
    local option_value="$2"

    if [ -z "$option_value" ]; then
        echo ""
        return
    fi

    # Replace CPU placeholder: #{cpu ...} -> #(${TMUX_SYSMON_BIN} cpu ...)
    option_value="$(echo "$option_value" | sed -E "s|#\{cpu([^}]*)\}|#(${TMUX_SYSMON_BIN} cpu\1)|g")"

    # Replace Memory placeholder: #{mem ...} -> #(${TMUX_SYSMON_BIN} mem ...)
    option_value="$(echo "$option_value" | sed -E "s|#\{mem([^}]*)\}|#(${TMUX_SYSMON_BIN} mem\1)|g")"

    # Replace Disk placeholder: #{disk ...} -> #(${TMUX_SYSMON_BIN} disk ...)
    option_value="$(echo "$option_value" | sed -E "s|#\{disk([^}]*)\}|#(${TMUX_SYSMON_BIN} disk\1)|g")"

    # Replace Battery placeholder: #{battery ...} -> #(${TMUX_SYSMON_BIN} battery ...)
    option_value="$(echo "$option_value" | sed -E "s|#\{battery([^}]*)\}|#(${TMUX_SYSMON_BIN} battery\1)|g")"

    echo "$option_value"
}

# Get current status options
LEFT_STATUS="$(tmux show-option -gqv status-left 2>/dev/null || echo "")"
RIGHT_STATUS="$(tmux show-option -gqv status-right 2>/dev/null || echo "")"

# Replace placeholders in both status options
NEW_LEFT="$(replace_placeholders "status-left" "$LEFT_STATUS")"
NEW_RIGHT="$(replace_placeholders "status-right" "$RIGHT_STATUS")"

# Apply the updated options (only if they were changed)
if [ -n "$NEW_LEFT" ] && [ "$NEW_LEFT" != "$LEFT_STATUS" ]; then
    tmux set-option -g status-left "$NEW_LEFT"
fi

if [ -n "$NEW_RIGHT" ] && [ "$NEW_RIGHT" != "$RIGHT_STATUS" ]; then
    tmux set-option -g status-right "$NEW_RIGHT"
fi

# Display success message
tmux display-message "tmux-sysmon (Rust): Integrated successfully with $TMUX_SYSMON_BIN"
