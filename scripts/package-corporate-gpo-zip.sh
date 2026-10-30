#!/usr/bin/env bash
# Kurumsal pilot: ADMX + ADML + örnek .reg → vitrin indirilebilir ZIP.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/apps/web/public/downloads"
ZIP="$OUT_DIR/Lertiva Arc-corporate-gpo.zip"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$OUT_DIR"
cp "$ROOT/deploy/gpo/Lertiva Arc.admx" "$STAGE/"
cp "$ROOT/deploy/gpo/README.md" "$STAGE/"
mkdir -p "$STAGE/en-US" "$STAGE/tr-TR"
cp "$ROOT/deploy/gpo/en-US/Lertiva Arc.adml" "$STAGE/en-US/"
cp "$ROOT/deploy/gpo/tr-TR/Lertiva Arc.adml" "$STAGE/tr-TR/"
cp "$ROOT/scripts/winlrt-corporate-policy-sample.reg" "$STAGE/"

(
  cd "$STAGE"
  rm -f "$ZIP"
  zip -rq "$ZIP" .
)

echo "OK: $ZIP ($(du -h "$ZIP" | cut -f1))"
