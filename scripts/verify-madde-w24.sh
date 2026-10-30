#!/usr/bin/env bash
# Madde W24: winlrtZIP Info-ZIP span (PK 07 08).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q ZIP_SPANNING_SIGNATURE "$ROOT/crates/winlrt-archive/src/zip_io.rs"
grep -q normalize_merged_split_zip "$ROOT/crates/winlrt-archive/src/zip_io.rs"
test -f "$ROOT/docs/qa/MADDE_W24_WINLRTZIP_INFOZIP_SPAN.md"
cargo test -p winlrt-archive split_zip_infozip --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w24_ --quiet
echo "OK: Madde W24 — winlrtZIP Info-ZIP span"
