#!/usr/bin/env bash
# Tauri kabuk düzenine uygun vitrin PNG'leri (headless Chrome).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/scripts/vitrin-screenshots"
OUT="$ROOT/apps/web/public/assets/gallery"
CHROME="${CHROME_BIN:-google-chrome}"

mkdir -p "$OUT"

shot() {
  local html="$1"
  local png="$2"
  local udir
  udir="$(mktemp -d)"
  timeout 45s "$CHROME" --headless=new --disable-gpu --hide-scrollbars \
    --user-data-dir="$udir" \
    --window-size=1024,720 \
    --screenshot="$png" \
    "file://$html" >/dev/null 2>&1 || true
  rm -rf "$udir"
  test -s "$png" || { echo "FAIL: $png" >&2; exit 1; }
  echo "OK: $png"
}

shot "$SRC/winlrtzip-mock.html" "$OUT/tauri-winlrtzip-main.png"
shot "$SRC/winlrtrar-mock.html" "$OUT/tauri-winlrtrar-main.png"
