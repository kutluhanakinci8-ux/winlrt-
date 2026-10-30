#!/usr/bin/env bash
# Madde W11: winlrtZIP I7 bozuk CRC kontrollu hata.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q ZIP_INTEGRITY_CRC_MSG "$ROOT/crates/winlrt-archive/src/zip_io.rs"
cargo test -p winlrt-archive zip_corrupt_crc --quiet
cargo test -p winlrt-archive --test winlrtzip_interop i7_ --quiet
echo "OK: Madde W11 - winlrtZIP CRC I7"
