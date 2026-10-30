#!/usr/bin/env bash
# Madde W16: Lertiva Zip P0×3 KPI kapisi scriptleri + ornek JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/scripts/evaluate-lertiva-zip-p0-gate.sh"
test -f "$ROOT/scripts/import-windows-zip-kpi-to-vitrin.sh"
test -f "$ROOT/docs/qa/MADDE_W16_WINLRTZIP_P0_KPI_GATE.md"
grep -q lertiva-zip-p0-gate "$ROOT/apps/web/public/benchmark/index.html"
test -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
cargo run -p lertiva-archive --release -- bench --all --json --zip-kpi >"$TMP/b.json"
cargo run -p lertiva-archive --release -- bench --all --profile fast --json --zip-kpi >"$TMP/f.json"
bash "$ROOT/scripts/evaluate-lertiva-zip-p0-gate.sh" "$TMP/b.json" "$TMP/f.json" "$TMP/p0.json"
jq -e '.p0.compression.gate != null and .p0.speed.gate != null and .p0.security.gate != null' "$TMP/p0.json" >/dev/null

echo "OK: Madde W16 — Lertiva Zip P0×3 KPI kapisi"
