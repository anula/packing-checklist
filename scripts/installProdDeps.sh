#!/bin/bash

# Fail if any command fails.
set -e

# Install everything needed to build the project.
echo "Prerequisites..."
apt update
apt install curl build-essential pkg-config -y

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

echo "Installing trunk..."
cargo install --locked trunk
