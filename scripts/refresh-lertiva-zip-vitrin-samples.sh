#!/usr/bin/env bash
# Lertiva Zip vitrin KPI örnekleri — bench --zip-kpi (Linux referans; resmi Windows CI).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
mkdir -p "$OUT"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
cargo run -p lertiva-archive --release -- bench --all --json --zip-kpi >"$TMP/balanced.json"
cargo run -p lertiva-archive --release -- bench --all --profile fast --json --zip-kpi >"$TMP/fast.json"

jq '{product: "Lertiva Zip", profile, platform, generated_at, source: "local-refresh", disclaimer: "Resmi Lertiva Zip satırı: Windows bench-report-windows artifact (zip_kpi).", kpi: .zip_kpi}' \
  "$TMP/balanced.json" >"$OUT/lertiva-zip-linux-balanced-kpi.json"
jq '{product: "Lertiva Zip", profile, platform, generated_at, source: "local-refresh", disclaimer: "Hız KPI Windows resmi workflow ile doğrulanır.", kpi: .zip_kpi}' \
  "$TMP/fast.json" >"$OUT/lertiva-zip-linux-fast-kpi.json"

for pair in balanced fast; do
  src="$OUT/lertiva-zip-linux-${pair}-kpi.json"
  dst="$OUT/lertiva-zip-macos-${pair}-kpi.json"
  jq '.platform = "macos / CI-placeholder (motor Linux proxy; resmi macOS bench W15+)" 
    | .disclaimer = "Resmi macOS zip_kpi: winlrt-zip-macos workflow artifact veya Mac agent bench --zip-kpi."' \
    "$src" >"$dst"
done

echo "OK: $OUT/lertiva-zip-linux-{balanced,fast}-kpi.json + lertiva-zip-macos-{balanced,fast}-kpi.json"
