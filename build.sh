#!/bin/bash
set -e

# Pre-built locally → skip build entirely
if [ -f dist/index.html ] && ls dist/*.wasm &>/dev/null 2>&1; then
  echo "📦 dist/ pre-built — skipping"
  exit 0
fi

# === Fallback: build on Cloudflare ===
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export PATH="$CARGO_HOME/bin:$PATH"
BIN="$CARGO_HOME/bin"
mkdir -p "$BIN"

# Restore caches
[ -d /opt/buildhome/cache/cargo ]  && cp -r /opt/buildhome/cache/cargo/.  "$CARGO_HOME/"  2>/dev/null || true
[ -d /opt/buildhome/cache/target ] && cp -r /opt/buildhome/cache/target/. ./target/        2>/dev/null || true

# Tailwind
[ ! -f "$BIN/tailwindcss" ] && curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.2.4/tailwindcss-linux-x64 \
  && chmod +x tailwindcss-linux-x64 && mv tailwindcss-linux-x64 "$BIN/tailwindcss"

# Rust
[ ! -f "$BIN/rustc" ] && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$CARGO_HOME/env"
rustup target add wasm32-unknown-unknown

# Trunk
[ ! -f "$BIN/trunk" ] && curl -sLO "https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz" \
  && tar xzf trunk-x86_64-unknown-linux-gnu.tar.gz && mv trunk "$BIN/" && rm trunk-x86_64-unknown-linux-gnu.tar.gz

LEKTOR_ENV=production trunk build --release

# Save caches
mkdir -p /opt/buildhome/cache/{cargo,target}
cp -r "$CARGO_HOME/registry" /opt/buildhome/cache/cargo/   2>/dev/null || true
cp -r target/release/.        /opt/buildhome/cache/target/ 2>/dev/null || true
