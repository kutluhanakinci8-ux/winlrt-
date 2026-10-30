#!/usr/bin/env bash
# Madde W10: Lertiva Zip I3 ZIP64.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q archive_indicates_zip64 "$ROOT/crates/lertiva-archive/src/zip_io.rs"
cargo test -p lertiva-archive zip64_smoke --quiet
cargo test -p lertiva-archive --test winlrtzip_interop i3_ --quiet
echo "OK: Madde W10 - Lertiva Zip ZIP64 I3"
