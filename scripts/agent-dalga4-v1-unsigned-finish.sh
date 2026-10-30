#!/usr/bin/env bash
# Dalga 4 kapanış: imzasız resmi v1.0.0 (TAURI secret yok — agent otomasyon).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="v1.0.0"
BETA_TAG="winlrt-rar-beta.41064653814"
ASSET="Lertiva Arc_0.1.0_x64-setup.exe"
URL="https://github.com/kutluhanakinci8-ux/winlrt-/releases/download/${BETA_TAG}/${ASSET}"
NOTES="$ROOT/docs/qa/V1_0_0_UNSIGNED_RELEASE.md"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

bash "$ROOT/scripts/run-dalga-smoke.sh"
bash "$ROOT/scripts/check-v1-gate-ready.sh"

curl -fsSL -o "$TMP/$ASSET" "$URL"

if ! git -C "$ROOT" rev-parse "$TAG" >/dev/null 2>&1; then
  git -C "$ROOT" tag -a "$TAG" -m "Lertiva Arc v1.0.0 (imzasız resmi — TAURI_SIGNING_* yok)"
  git -C "$ROOT" push origin "$TAG"
fi

if ! gh release view "$TAG" --repo kutluhanakinci8-ux/winlrt- >/dev/null 2>&1; then
  gh release create "$TAG" \
    --repo kutluhanakinci8-ux/winlrt- \
    --title "Lertiva Arc v1.0.0 (Windows, imzasız resmi)" \
    --notes-file "$NOTES" \
    "$TMP/$ASSET"
else
  echo "Release $TAG mevcut"
fi

bash "$ROOT/scripts/generate-pilot-status-json.sh"
echo "OK: v1.0.0 imzasız resmi release"
