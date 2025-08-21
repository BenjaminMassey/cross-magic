#!/bin/bash
set -e  # exit on error

# Get datetime in format YYYY-MM-DD_HH-MM-SS
DATETIME=$(date +"%Y-%m-%d_%H-%M-%S")

# Run cargo build
cargo build --release

# Create build directory
BUILDDIR="deployments/linux/build_$DATETIME"
mkdir -p "$BUILDDIR"

# Copy required folders
cp -r "llama-lin" "$BUILDDIR/llama-linux"
cp -r "llama-model" "$BUILDDIR/llama-model"

# Copy compiled executables
cp target/release/*.exe "$BUILDDIR" 2>/dev/null || true
cp target/release/* "$BUILDDIR"