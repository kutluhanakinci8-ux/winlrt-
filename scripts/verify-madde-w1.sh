#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo test -p lertiva-archive --quiet
cargo build -p lertiva-archive --release --bin lertiva-zip --quiet
test -x "$ROOT/target/release/lertiva-zip"
"$ROOT/target/release/lertiva-zip" --help | grep -q create
test -f "$ROOT/docs/qa/MADDE_W1_WINLRTZIP_CLI.md"

echo "OK: Madde W1 — lertiva-zip CLI"
