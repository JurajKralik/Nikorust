#!/bin/bash
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(pwd)")"
cd "$REPO_ROOT"

echo "Repo root: $REPO_ROOT"

if [[ -f "utils/version_control.py" ]]; then
    echo "Running version bump..."
    python3 utils/version_control.py
else
    echo "⚠️ Error: utils/version_control.py not found, skipping version bump"
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
    
    cd "$REPO_ROOT"
    if [[ -f "utils/upload_bot.py" ]]; then
        echo "Uploading bot to AI Arena..."
        if [[ -f ".env" ]]; then
            source .env
            export AIARENA_API_KEY
        fi
        python3 utils/upload_bot.py
    else
        echo "⚠️ Error: utils/upload_bot.py not found, skipping upload"
    fi
else
    echo "⚠️ Error: no binary at $BIN_DEST"
fi
