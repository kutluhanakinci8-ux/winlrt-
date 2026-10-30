#!/usr/bin/env bash
# Madde W20: winlrtZIP Split ZIP smoke (algilama).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q archive_indicates_split_zip "$ROOT/crates/winlrt-archive/src/zip_io.rs"
test -f "$ROOT/docs/qa/MADDE_W20_WINLRTZIP_SPLIT_ZIP.md"
cargo test -p winlrt-archive split_zip --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w20_ --quiet
echo "OK: Madde W20 — winlrtZIP Split ZIP smoke"
