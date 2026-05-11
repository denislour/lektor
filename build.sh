#!/bin/bash
set -e

if [ -f dist/index.html ] && ls dist/*.wasm &>/dev/null 2>&1; then
  echo "dist/ pre-built — skipping"
  exit 0
fi

echo "dist/ not pre-built. Run 'trunk build --release' locally first."
exit 1
