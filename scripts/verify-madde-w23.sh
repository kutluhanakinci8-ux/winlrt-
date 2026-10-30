#!/usr/bin/env bash
# Madde W23: Lertiva Zip imzali arsiv (Ed25519 sidecar).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q RequireSignedArchive "$ROOT/deploy/gpo/Lertiva Zip.admx"
grep -q sign_zip_archive "$ROOT/crates/lertiva-archive/src/zip_io.rs"
test -f "$ROOT/docs/qa/MADDE_W23_WINLRTZIP_SIGNED_ARCHIVE.md"
cargo test -p lertiva-archive archive_sign:: --quiet
cargo test -p lertiva-archive --test winlrtzip_interop w23_ --quiet
echo "OK: Madde W23 — Lertiva Zip imzalı arşiv"
