#!/bin/bash
set -e  # exit on error

# Go to repo
cd /home/dax/Bob/Repositories/Nikorust

# Build with musl target
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl --features ladder

# Paths
BUILD_DIR="target/x86_64-unknown-linux-musl/release"
BIN_SRC="$BUILD_DIR/Nikolaj"
BIN_DEST="$BUILD_DIR/RustyNikolaj"
ZIP_NAME="RustyNikolaj.zip"

# Rename binary
cp "$BIN_SRC" "$BIN_DEST"

# Create zip
cd "$BUILD_DIR"
zip -r "$ZIP_NAME" "RustyNikolaj"

# Move zip to /
mv "$ZIP_NAME" /home/dax/Bob/Repositories/Nikorust
echo "Build complete! 🦾"