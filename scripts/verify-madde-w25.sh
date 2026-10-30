#!/usr/bin/env bash
# Madde W25: Lertiva Zip guvenli paylasim (KVKK smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q create_secure_share_pack "$ROOT/crates/lertiva-archive/src/secure_share.rs"
grep -q SharePack "$ROOT/crates/lertiva-archive/src/zip_product_cli.rs"
test -f "$ROOT/docs/qa/MADDE_W25_WINLRTZIP_SECURE_SHARE.md"
cargo test -p lertiva-archive secure_share:: --quiet
cargo test -p lertiva-archive --test lertiva_zip_interop w25_ --quiet
echo "OK: Madde W25 — Lertiva Zip güvenli paylaşım (KVKK)"
