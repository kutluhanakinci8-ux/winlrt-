#!/usr/bin/env bash
# Madde W21: winlrtZIP Split ZIP birlestirme (acma).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q split_zip_ordered_volumes "$ROOT/crates/winlrt-archive/src/zip_io.rs"
grep -q SplitZipOpen "$ROOT/crates/winlrt-archive/src/zip_io.rs"
test -f "$ROOT/docs/qa/MADDE_W21_WINLRTZIP_SPLIT_ZIP_MERGE.md"
cargo test -p winlrt-archive split_zip --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w21_ --quiet
echo "OK: Madde W21 — winlrtZIP Split ZIP birleştirme"
