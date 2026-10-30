#!/usr/bin/env bash
# Yerel nokta yedek — aktif vitrin kaynaklarının kopyası (git dışı çalışma alanı değil).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
DEST="$ROOT/archive/snapshots/$STAMP"

mkdir -p "$DEST"
for path in apps/web/public apps/web/scripts docs/WINLRTRAR_PLAN.md docs/WINLRTRAR_PHASE0.md; do
  if [ -e "$ROOT/$path" ]; then
    mkdir -p "$DEST/$(dirname "$path")"
    cp -a "$ROOT/$path" "$DEST/$path"
  fi
done

echo "OK: $DEST"
ls -dt "$ROOT/archive/snapshots"/* 2>/dev/null | tail -n +6 | xargs -r rm -rf
