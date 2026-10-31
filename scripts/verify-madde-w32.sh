#!/usr/bin/env bash
# Madde W32: Lertiva Zip mağaza + mobil pilot hazırlık kapısı (W27+W26+W31+vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q evaluate_store_pilot_gate "$ROOT/crates/lertiva-archive/src/store_pilot_gate.rs"
grep -q StorePilotGate "$ROOT/crates/lertiva-archive/src/zip_product_cli.rs"
test -f "$ROOT/docs/qa/MADDE_W32_WINLRTZIP_STORE_PILOT.md"
cargo test -p lertiva-archive store_pilot_gate:: --quiet
bash "$ROOT/scripts/evaluate-lertiva-zip-store-pilot-gate.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-pilot-gate.json"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-pilot-gate.json" >/dev/null
cargo test -p lertiva-archive --test lertiva_zip_interop w32_ --quiet
cargo check -p lertiva-zip --quiet
echo "OK: Madde W32 — Lertiva Zip mağaza/mobil pilot hazırlık"
