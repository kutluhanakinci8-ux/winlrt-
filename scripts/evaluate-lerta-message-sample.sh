#!/usr/bin/env bash
# W30: örnek Lerta mesaj eki webhook JSON (vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"
echo "lerta-message-sample" > "$TMP/note.txt"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-message-v1.json}"
cargo run -q -p lertiva-archive --bin lertiva-zip -- --json lerta-message-prepare "$TMP/note.txt" "$TMP/mesaj.wlrt" > "$OUT"
echo "OK: $OUT"
rm -rf "$TMP"
