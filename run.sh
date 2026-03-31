#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -f "$SCRIPT_DIR/.env" ]; then
    set -a 
    source "$SCRIPT_DIR/.env"
    set +a
fi

cd "$SCRIPT_DIR"
if [ -f "$HOME/.cargo/env" ]; then
    # Ensure Cargo/rustup bin is on PATH for non-interactive shells
    # (this file is created by rustup-init)
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

# Create logs directory if it doesn't exist
mkdir -p logs

# Generate timestamp for unique log file
LOG_FILE="logs/bot_$(date +%Y%m%d_%H%M%S).log"

echo "Starting bot... Log file: $LOG_FILE"
cargo run --features wine_sc2 2>&1 | tee "$LOG_FILE"
