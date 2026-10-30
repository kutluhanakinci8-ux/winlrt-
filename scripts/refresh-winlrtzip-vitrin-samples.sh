#!/usr/bin/env bash
# winlrtZIP vitrin KPI örnekleri — bench --zip-kpi (Linux referans; resmi Windows CI).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
mkdir -p "$OUT"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
cargo run -p winlrt-archive --release -- bench --all --json --zip-kpi >"$TMP/balanced.json"
cargo run -p winlrt-archive --release -- bench --all --profile fast --json --zip-kpi >"$TMP/fast.json"

jq '{product: "winlrtZIP", profile, platform, generated_at, source: "local-refresh", disclaimer: "Resmi winlrtZIP satırı: Windows bench-report-windows artifact (zip_kpi).", kpi: .zip_kpi}' \
  "$TMP/balanced.json" >"$OUT/winlrtzip-linux-balanced-kpi.json"
jq '{product: "winlrtZIP", profile, platform, generated_at, source: "local-refresh", disclaimer: "Hız KPI Windows resmi workflow ile doğrulanır.", kpi: .zip_kpi}' \
  "$TMP/fast.json" >"$OUT/winlrtzip-linux-fast-kpi.json"

echo "OK: $OUT/winlrtzip-linux-{balanced,fast}-kpi.json"
