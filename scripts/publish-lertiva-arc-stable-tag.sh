#!/usr/bin/env bash
# Annotated tag v1.0.x → lertiva-arc-windows-stable workflow.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.1}"
VER="${TAG#v}"
CONF="$(grep -E '"version"' "$ROOT/apps/lertiva-arc/src-tauri/tauri.conf.json" | head -1 | sed 's/.*"\([0-9.]*\)".*/\1/')"
if [ "$CONF" != "$VER" ]; then
  echo "tauri.conf version ($CONF) != tag ($VER)" >&2
  exit 1
fi
git -C "$ROOT" fetch origin main
git -C "$ROOT" tag -a "$TAG" -m "Lertiva Arc $VER Windows stable" 2>/dev/null || git -C "$ROOT" tag -f -a "$TAG" -m "Lertiva Arc $VER Windows stable"
git -C "$ROOT" push origin "$TAG"
echo "OK: tag $TAG pushed — GitHub Actions: lertiva-arc-windows-stable"
