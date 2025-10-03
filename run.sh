#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -f "$SCRIPT_DIR/.env" ]; then
    export $(grep -v '^#' "$SCRIPT_DIR/.env" | xargs)
fi

export WINEDEBUG=${WINEDEBUG:--all}
export SC2PATH=${SC2PATH:-"$HOME/Battle.net/StarcraftII"}

echo "Using SC2 at: $SC2PATH"
echo "Repo path: $SCRIPT_DIR"

cd "$SCRIPT_DIR"
cargo run --features wine_sc2
