#!/usr/bin/env bash
# Mac: evaluate scriptlerinin degistirdigi gate JSON'lari geri al, main cek.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

patterns=(
  apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json
  apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json
  apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json
)

for f in "${patterns[@]}"; do
  if git diff --quiet "$f" 2>/dev/null; then
    continue
  fi
  echo "Geri aliniyor (yerel evaluate): $f"
  git checkout -- "$f" 2>/dev/null || git restore "$f" 2>/dev/null || true
done

git pull origin main
echo "OK: main guncel — bash scripts/mac-g2-g3-quickcheck.sh"
