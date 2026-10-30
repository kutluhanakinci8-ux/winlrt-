#!/usr/bin/env bash
# W29: örnek Lerta API durum JSON (vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-api-v1.json}"
export WINLRT_LERTA_API_DRY_RUN=1
cargo run -q -p lertiva-archive --bin lertiva-zip -- --json lerta-api-status > "$OUT"
echo "OK: $OUT"
