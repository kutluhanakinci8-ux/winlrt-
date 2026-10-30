#!/usr/bin/env bash
# Madde W11: Lertiva Zip I7 bozuk CRC kontrollu hata.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q ZIP_INTEGRITY_CRC_MSG "$ROOT/crates/lertiva-archive/src/zip_io.rs"
cargo test -p lertiva-archive zip_corrupt_crc --quiet
cargo test -p lertiva-archive --test lertiva_zip_interop i7_ --quiet
echo "OK: Madde W11 - Lertiva Zip CRC I7"
