#!/usr/bin/env bash
# Madde W25: winlrtZIP guvenli paylasim (KVKK smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q create_secure_share_pack "$ROOT/crates/winlrt-archive/src/secure_share.rs"
grep -q SharePack "$ROOT/crates/winlrt-archive/src/zip_product_cli.rs"
test -f "$ROOT/docs/qa/MADDE_W25_WINLRTZIP_SECURE_SHARE.md"
cargo test -p winlrt-archive secure_share:: --quiet
cargo test -p winlrt-archive --test winlrtzip_interop w25_ --quiet
echo "OK: Madde W25 — winlrtZIP güvenli paylaşım (KVKK)"
