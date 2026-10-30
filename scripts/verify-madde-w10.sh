#!/usr/bin/env bash
# Madde W10: winlrtZIP I3 ZIP64.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q archive_indicates_zip64 "$ROOT/crates/winlrt-archive/src/zip_io.rs"
cargo test -p winlrt-archive zip64_smoke --quiet
cargo test -p winlrt-archive --test winlrtzip_interop i3_ --quiet
echo "OK: Madde W10 - winlrtZIP ZIP64 I3"
