#!/usr/bin/env bash
# W31: birleşik Lerta entegrasyon durumu JSON (vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-integration-v1.json}"
export WINLRT_LERTA_API_DRY_RUN=1
cargo run -q -p winlrt-archive --bin winlrt-zip -- --json lerta-integration-status > "$OUT"
echo "OK: $OUT"
