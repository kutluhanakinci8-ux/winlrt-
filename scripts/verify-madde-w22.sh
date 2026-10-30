#!/usr/bin/env bash
# Madde W22: Lertiva Zip Split ZIP yazma.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q create_split_with_profile "$ROOT/crates/lertiva-archive/src/zip_io.rs"
grep -q CreateSplit "$ROOT/crates/lertiva-archive/src/zip_product_cli.rs"
test -f "$ROOT/docs/qa/MADDE_W22_WINLRTZIP_SPLIT_ZIP_WRITE.md"
cargo test -p lertiva-archive split_zip_write --quiet
cargo test -p lertiva-archive --test winlrtzip_interop w22_ --quiet
echo "OK: Madde W22 — Lertiva Zip Split ZIP yazma"
