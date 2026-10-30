#!/usr/bin/env bash
# W27: Lertiva Zip mağaza / güncelleme kapısı JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
OUT="${1:-$ROOT/store-gate-lertiva-zip.json}"
cargo run -q -p lertiva-archive --bin lertiva-zip -- --json store-gate --workspace "$ROOT" > "$OUT"
echo "OK: $OUT"
