#!/usr/bin/env bash
# W28: örnek Lerta posta eki webhook JSON (vitrin).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"
echo "lerta-sample" > "$TMP/note.txt"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/winlrtzip-lerta-mail-v1.json}"
cargo run -q -p lertiva-archive --bin lertiva-zip -- --json lerta-mail-prepare "$TMP/note.txt" "$TMP/ek.zip" > "$OUT"
echo "OK: $OUT"
rm -rf "$TMP"
