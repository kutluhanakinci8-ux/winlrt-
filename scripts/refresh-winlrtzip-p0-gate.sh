#!/usr/bin/env bash
# Yerel Linux proxy: bench zip_kpi + P0×3 JSON (vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cd "$ROOT"
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
cargo run -p lertiva-archive --release -- bench --all --json --zip-kpi >"$TMP/balanced.json"
cargo run -p lertiva-archive --release -- bench --all --profile fast --json --zip-kpi >"$TMP/fast.json"
bash "$ROOT/scripts/refresh-winlrtzip-vitrin-samples.sh"
bash "$ROOT/scripts/evaluate-winlrtzip-p0-gate.sh" "$TMP/balanced.json" "$TMP/fast.json"

echo "OK: Lertiva Zip P0×3 vitrin güncellendi (Linux proxy)"
