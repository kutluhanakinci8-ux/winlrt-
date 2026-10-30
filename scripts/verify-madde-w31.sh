#!/usr/bin/env bash
# Madde W31: Lertiva Zip Lerta birleşik staging kapısı.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q lerta_integration_status_from_env "$ROOT/crates/lertiva-archive/src/lerta_integration.rs"
grep -q lerta_integration_status "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"
grep -q LertaIntegrationStatus "$ROOT/crates/lertiva-archive/src/zip_product_cli.rs"
test -f "$ROOT/docs/qa/MADDE_W31_WINLRTZIP_LERTA_INTEGRATION.md"
test -f "$ROOT/docs/qa/lerta-integration-v1.schema.json"
cargo test -p lertiva-archive lerta_integration:: --quiet
bash "$ROOT/scripts/evaluate-lerta-integration-sample.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-integration-v1.json"
cargo test -p lertiva-archive --test winlrtzip_interop w31_ --quiet
cargo check -p lertiva-zip --quiet
echo "OK: Madde W31 — Lertiva Zip Lerta birleşik staging"
