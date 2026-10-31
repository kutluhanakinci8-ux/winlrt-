#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-pilot-gate.json}"
export WINLRT_LERTA_API_DRY_RUN=1
cargo run -q -p lertiva-archive --bin lertiva-zip -- --json store-pilot-gate --workspace "$ROOT" > "$OUT"
echo "OK: $OUT"
