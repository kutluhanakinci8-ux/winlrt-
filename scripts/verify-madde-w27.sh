#!/usr/bin/env bash
# Madde W27: Lertiva Zip mağaza olgunluğu smoke.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q evaluate_store_gate "$ROOT/crates/lertiva-archive/src/store_gate.rs"
test -f "$ROOT/docs/qa/MADDE_W27_WINLRTZIP_STORE_GATE.md"
test -f "$ROOT/docs/qa/WINLRTZIP_STORE_DATA_SAFETY.md"
cargo test -p lertiva-archive store_gate:: --quiet
bash "$ROOT/scripts/evaluate-lertiva-zip-store-gate.sh" "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-gate.json"
test -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-gate.json"
cargo test -p lertiva-archive --test lertiva_zip_interop w27_ --quiet
echo "OK: Madde W27 — Lertiva Zip mağaza kapısı"
