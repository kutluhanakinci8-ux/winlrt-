#!/usr/bin/env bash
# G2 vitrin: SmartScreen + NSIS kurulum PNG → incoming/ (Chrome headless).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/scripts/vitrin-screenshots"
IN="$ROOT/apps/web/public/assets/gallery/incoming"
CHROME="${CHROME_BIN:-google-chrome}"

mkdir -p "$IN"

shot() {
  local html="$1"
  local png="$2"
  local udir
  udir="$(mktemp -d)"
  timeout 60s "$CHROME" --headless=new --disable-gpu --hide-scrollbars \
    --user-data-dir="$udir" \
    --window-size=1920,1080 \
    --force-device-scale-factor=2 \
    --screenshot="$png" \
    "file://$html" >/dev/null 2>&1 || true
  rm -rf "$udir"
  test -s "$png" || { echo "FAIL: $png" >&2; exit 1; }
  echo "OK: $png ($(wc -c <"$png" | tr -d ' ') B)"
}

shot "$SRC/smartscreen-incoming.html" "$IN/smartscreen-lertiva.png"
shot "$SRC/lertiva-zip-installer-mock.html" "$IN/installer-lertiva-zip-setup.png"
shot "$SRC/lertiva-arc-installer-mock.html" "$IN/installer-lertiva-arc-setup.png"

bash "$ROOT/scripts/import-real-installer-screenshots.sh"
echo "OK: G2 incoming vitrin PNG üretildi"
