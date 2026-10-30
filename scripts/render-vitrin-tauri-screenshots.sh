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

shot "$SRC/lertiva-zip-mock.html" "$OUT/tauri-lertiva-zip-main.png"
shot "$SRC/lertiva-arc-mock.html" "$OUT/tauri-lertiva-arc-main.png"
shot "$SRC/lertiva-arc-installer-mock.html" "$OUT/installer-lertiva-arc-setup.png"
shot "$SRC/lertiva-zip-installer-mock.html" "$OUT/installer-lertiva-zip-setup.png"
shot "$SRC/smartscreen-mock.html" "$OUT/smartscreen-lertiva.png"
