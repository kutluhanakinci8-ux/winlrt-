#!/usr/bin/env bash
# Madde W27: winlrtZIP mağaza olgunluğu smoke.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q evaluate_store_gate "$ROOT/crates/winlrt-archive/src/store_gate.rs"
test -f "$ROOT/docs/qa/MADDE_W27_WINLRTZIP_STORE_GATE.md"
test -f "$ROOT/docs/qa/WINLRTZIP_STORE_DATA_SAFETY.md"
cargo test -p winlrt-archive store_gate:: --quiet
bash "$ROOT/scripts/evaluate-winlrtzip-store-gate.sh" "$ROOT/apps/web/public/benchmark/samples/winlrtzip-store-gate.json"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-store-gate.json"
cargo test -p winlrt-archive --test winlrtzip_interop w27_ --quiet
echo "OK: Madde W27 — winlrtZIP mağaza kapısı"
