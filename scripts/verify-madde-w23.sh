#!/usr/bin/env bash
# Madde W23: winlrtZIP imzali arsiv (Ed25519 sidecar).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q RequireSignedArchive "$ROOT/deploy/gpo/winlrtZIP.admx"
grep -q sign_zip_archive "$ROOT/crates/winlrt-archive/src/zip_io.rs"
test -f "$ROOT/docs/qa/MADDE_W23_WINLRTZIP_SIGNED_ARCHIVE.md"
cargo test -p winlrt-archive archive_sign:: --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w23_ --quiet
echo "OK: Madde W23 — winlrtZIP imzalı arşiv"
