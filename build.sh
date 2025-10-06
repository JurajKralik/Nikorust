#!/bin/bash
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(pwd)")"
cd "$REPO_ROOT"

echo "Repo root: $REPO_ROOT"

if [[ -f "version_control.py" ]]; then
    echo "Running version bump..."
    python3 version_control.py
else
    echo "⚠️ Error: version_control.py not found, skipping version bump"
fi

rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl --features ladder

BUILD_DIR="target/x86_64-unknown-linux-musl/release"
BIN_SRC="$BUILD_DIR/Nikolaj"
BIN_DEST="$BUILD_DIR/RustyNikolaj"
ZIP_NAME="RustyNikolaj.zip"

if [[ -f "$BIN_SRC" ]]; then
    cp "$BIN_SRC" "$BIN_DEST"
    echo "Binary copied to $BIN_DEST"
else
    echo "⚠️ Error: binary not found at $BIN_SRC"
fi

if [[ -f "$BIN_DEST" ]]; then
    cd "$BUILD_DIR"
    zip -r "$ZIP_NAME" "RustyNikolaj"
    mv "$ZIP_NAME" "$REPO_ROOT/"
    echo "Build complete! 🦾"
else
    echo "⚠️ Error: no binary at $BIN_DEST"
fi
