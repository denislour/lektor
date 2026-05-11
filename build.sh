#!/bin/bash
set -e

curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mkdir -p /opt/buildhome/.cargo/bin
mv tailwindcss-linux-x64 /opt/buildhome/.cargo/bin/tailwindcss
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install trunk
export LEKTOR_ENV=production
trunk build --release
