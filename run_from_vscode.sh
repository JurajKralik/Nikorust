#!/bin/bash

# This script works from Flatpak VS Code by running on the host system
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

flatpak-spawn --host bash -c "cd '$SCRIPT_DIR' && source \$HOME/.cargo/env && ./run.sh"
