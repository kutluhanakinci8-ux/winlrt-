#!/usr/bin/env bash
# Vitrin /benchmark/samples/*.json — yerel bench çıktısından KPI özeti (tam suite değil).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
mkdir -p "$OUT"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
cargo run -p winlrt-archive --release -- bench --all --json >"$TMP/balanced.json"
cargo run -p winlrt-archive --release -- bench --all --profile fast --json >"$TMP/fast.json"

jq '{profile, platform, generated_at, source: "local-refresh", disclaimer: "Windows resmi KPI için bench-report-windows artifact.", kpi}' \
  "$TMP/balanced.json" >"$OUT/linux-balanced-kpi.json"
jq '{profile, platform, generated_at, source: "local-refresh", disclaimer: "Hız kapısı Windows resmi workflow ile doğrulanır.", kpi}' \
  "$TMP/fast.json" >"$OUT/linux-fast-kpi.json"

echo "Yazıldı: $OUT/linux-balanced-kpi.json"
echo "Yazıldı: $OUT/linux-fast-kpi.json"
