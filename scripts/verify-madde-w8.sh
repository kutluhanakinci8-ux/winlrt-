#!/usr/bin/env bash
# Madde W8: winlrtZIP I4 AES-256 WinZip uyumlu interop.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
grep -q i4_aes256_roundtrip "$ROOT/crates/winlrt-archive/tests/winlrtzip_interop.rs"
grep -q AesMode::Aes256 "$ROOT/crates/winlrt-archive/src/zip_io.rs"
cargo test -p winlrt-archive --test winlrtzip_interop i4_ --quiet
if ! command -v 7z >/dev/null 2>&1; then
  echo "NOT: 7z yok — i4_7zip testi icinde skip (CI main-pipeline p7zip-full kurar)"
fi
echo "OK: Madde W8 - winlrtZIP AES I4 interop"
