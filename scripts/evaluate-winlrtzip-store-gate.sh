#!/usr/bin/env bash
# W27: winlrtZIP mağaza / güncelleme kapısı JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
OUT="${1:-$ROOT/store-gate-winlrtzip.json}"
cargo run -q -p winlrt-archive --bin winlrt-zip -- --json store-gate --workspace "$ROOT" > "$OUT"
echo "OK: $OUT"
