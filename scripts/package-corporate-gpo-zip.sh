#!/usr/bin/env bash
# Kurumsal pilot: ADMX + ADML + örnek .reg → vitrin indirilebilir ZIP.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/apps/web/public/downloads"
ZIP="$OUT_DIR/lertiva-arc-corporate-gpo.zip"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$OUT_DIR"
cp "$ROOT/deploy/gpo/lertiva-arc.admx" "$STAGE/"
cp "$ROOT/deploy/gpo/README.md" "$STAGE/"
mkdir -p "$STAGE/en-US" "$STAGE/tr-TR"
cp "$ROOT/deploy/gpo/en-US/lertiva-arc.adml" "$STAGE/en-US/"
cp "$ROOT/deploy/gpo/tr-TR/lertiva-arc.adml" "$STAGE/tr-TR/"
cp "$ROOT/scripts/lertiva-arc-corporate-policy-sample.reg" "$STAGE/"

(
  cd "$STAGE"
  rm -f "$ZIP"
  zip -rq "$ZIP" .
)

echo "OK: $ZIP ($(du -h "$ZIP" | cut -f1))"
