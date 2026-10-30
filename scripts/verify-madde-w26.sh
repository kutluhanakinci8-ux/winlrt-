#!/usr/bin/env bash
# Madde W26: winlrtZIP mobil okuyucu smoke.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q mobile_reader "$ROOT/crates/winlrt-archive/src/lib.rs"
grep -q mobile_inspect_zip "$ROOT/apps/winlrt-zip/src-tauri/src/lib.rs"
test -f "$ROOT/docs/qa/mobile-reader-v1.schema.json"
test -f "$ROOT/docs/qa/MADDE_W26_WINLRTZIP_MOBILE_READER.md"
cargo test -p winlrt-archive mobile_reader:: --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w26_ --quiet
cargo check -p winlrt-zip --quiet
echo "OK: Madde W26 — winlrtZIP mobil okuyucu smoke"
