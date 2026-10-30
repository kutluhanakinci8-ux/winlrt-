#!/usr/bin/env bash
# Madde W9: winlrtZIP I5 UTF-8 Turkce dosya adlari.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q zip_entry_name_from_path "$ROOT/crates/winlrt-archive/src/zip_io.rs"
cargo test -p winlrt-archive --lib zip_turkish_utf8 --quiet
cargo test -p winlrt-archive --test winlrtzip_interop i5_ --quiet
echo "OK: Madde W9 - winlrtZIP UTF-8 I5"
