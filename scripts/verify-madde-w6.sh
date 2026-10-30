#!/usr/bin/env bash
# Madde W6: winlrtZIP ZIP benchmark satırı + zip_kpi CI.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
grep -q zip_kpi "$ROOT/crates/winlrt-archive/src/bench.rs"
grep -q evaluate_zip_product_kpi "$ROOT/crates/winlrt-archive/src/bench.rs"
grep -q WINLRT_BENCH_INCLUDE_WINZIP "$ROOT/.github/workflows/bench-windows.yml"
grep -q evaluate-bench-zip-kpi.ps1 "$ROOT/.github/workflows/bench-windows.yml"
grep -q 'zip-kpi' "$ROOT/.github/workflows/bench-windows.yml"
test -f "$ROOT/scripts/evaluate-bench-zip-kpi.ps1"
test -f "$ROOT/apps/web/public/benchmark/samples/winlrtzip-linux-balanced-kpi.json"
cargo test -p winlrt-archive --lib bench::tests::zip_product --quiet
echo "OK: Madde W6 - winlrtZIP bench zip_kpi + CI"
