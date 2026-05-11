#!/bin/bash
set -e

export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
BIN="$CARGO_HOME/bin"

# === Restore cache ===
if [ -d /opt/buildhome/cache/cargo ]; then
  cp -r /opt/buildhome/cache/cargo/. "$CARGO_HOME/" 2>/dev/null || true
fi
if [ -d /opt/buildhome/cache/target ]; then
  cp -r /opt/buildhome/cache/target/. ./target/ 2>/dev/null || true
fi

mkdir -p "$BIN"

# Tailwind binary
if [ ! -f "$BIN/tailwindcss" ]; then
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64
  chmod +x tailwindcss-linux-x64 && mv tailwindcss-linux-x64 "$BIN/tailwindcss"
fi

# Rust toolchain (check binary directly, avoids PATH issues)
if [ ! -f "$BIN/rustc" ]; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
. "$CARGO_HOME/env"
rustup target add wasm32-unknown-unknown

# Trunk (pre-compiled binary, not compiled from source)
if [ ! -f "$BIN/trunk" ]; then
  curl -sLO "https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz"
  tar xzf trunk-x86_64-unknown-linux-gnu.tar.gz && mv trunk "$BIN/" && rm trunk-x86_64-unknown-linux-gnu.tar.gz
fi

export LEKTOR_ENV=production
trunk build --release

# === Save cache for next build ===
mkdir -p /opt/buildhome/cache/cargo /opt/buildhome/cache/target
cp -r "$CARGO_HOME/registry" /opt/buildhome/cache/cargo/ 2>/dev/null || true
cp -r target/release/. /opt/buildhome/cache/target/ 2>/dev/null || true
