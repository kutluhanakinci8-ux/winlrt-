#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo test -p winlrt-archive --quiet
cargo build -p winlrt-archive --release --bin winlrt-zip --quiet
test -x "$ROOT/target/release/winlrt-zip"
"$ROOT/target/release/winlrt-zip" --help | grep -q create
test -f "$ROOT/docs/qa/MADDE_W1_WINLRTZIP_CLI.md"

echo "OK: Madde W1 — winlrt-zip CLI"
