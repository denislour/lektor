#!/bin/bash
set -e

# Download tailwindcss binary
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mkdir -p /opt/buildhome/.cargo/bin
mv tailwindcss-linux-x64 /opt/buildhome/.cargo/bin/tailwindcss

# Setup Rust — skip if cargo already installed
if ! command -v cargo &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

# Install trunk as pre-compiled binary (avoids 7+ min compile from source)
TRUNK_VERSION="0.21.14"
if ! command -v trunk &> /dev/null; then
    curl -sLO "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz"
    tar xzf "trunk-x86_64-unknown-linux-gnu.tar.gz"
    mv trunk /opt/buildhome/.cargo/bin/
    rm -f "trunk-x86_64-unknown-linux-gnu.tar.gz"
fi

export LEKTOR_ENV=production
trunk build --release
