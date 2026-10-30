#!/usr/bin/env bash
# Dalga 4: v1.0.0-rc1 GitHub Release (imzasız beta exe ekleri).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="v1.0.0-rc1"
BETA_TAG="winlrt-rar-beta.41064653814"
ASSET="Lertiva Arc_0.1.0_x64-setup.exe"
URL="https://github.com/kutluhanakinci8-ux/winlrt-/releases/download/${BETA_TAG}/${ASSET}"
NOTES="$ROOT/docs/qa/V1_RELEASE_DRAFT.md"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if gh release view "$TAG" >/dev/null 2>&1; then
  echo "Release $TAG zaten var — atlanıyor"
  exit 0
fi

curl -fsSL -o "$TMP/$ASSET" "$URL"
gh release create "$TAG" \
  --repo kutluhanakinci8-ux/winlrt- \
  --title "Lertiva Arc v1.0.0-rc1 (imzasız beta)" \
  --notes-file "$NOTES" \
  --prerelease \
  "$TMP/$ASSET"

echo "OK: GitHub Release $TAG"
