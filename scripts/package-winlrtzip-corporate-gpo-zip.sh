#!/usr/bin/env bash
# Lertiva Zip kurumsal ADMX paketi → vitrin indirilebilir ZIP.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/apps/web/public/downloads"
ZIP="$OUT_DIR/Lertiva Zip-corporate-gpo.zip"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$OUT_DIR"
cp "$ROOT/deploy/gpo/Lertiva Zip.admx" "$STAGE/"
cp "$ROOT/deploy/gpo/README-Lertiva Zip.md" "$STAGE/README.md"
mkdir -p "$STAGE/en-US" "$STAGE/tr-TR"
cp "$ROOT/deploy/gpo/en-US/Lertiva Zip.adml" "$STAGE/en-US/"
cp "$ROOT/deploy/gpo/tr-TR/Lertiva Zip.adml" "$STAGE/tr-TR/"
cp "$ROOT/scripts/winlrtzip-corporate-policy-sample.reg" "$STAGE/"

(
  cd "$STAGE"
  rm -f "$ZIP"
  zip -rq "$ZIP" .
)

echo "OK: $ZIP ($(du -h "$ZIP" | cut -f1))"
