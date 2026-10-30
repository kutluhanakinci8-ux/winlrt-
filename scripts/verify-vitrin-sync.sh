#!/usr/bin/env bash
# Vitrin metni eski plan ifadelerini taşımamalı (repo ile hizalı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IDX="$ROOT/apps/web/public/index.html"

for bad in "Faz 0 plan" "RAR F2 sonrası" "İlk sürümler yakında" "yalnızca plan"; do
  if grep -q "$bad" "$IDX"; then
    echo "FAIL: index.html hâlâ eski ifade içeriyor: $bad"
    exit 1
  fi
done

grep -q 'id="durum"' "$IDX"
grep -q 'winlrtzip-p0-gate.json' "$IDX"
grep -q 'Post-v3' "$IDX"
grep -q 'styles.css?v=5' "$IDX"
test -f "$ROOT/apps/web/public/js/site.js"
test -f "$ROOT/apps/web/public/js/i18n.js"
test -f "$ROOT/apps/web/public/hakkimizda/index.html"
test -f "$ROOT/apps/web/public/kvkk/index.html"
test -f "$ROOT/apps/web/public/assets/logo-mark.svg"
grep -q 'id="galeri"' "$IDX"
grep -q 'data-theme' "$ROOT/apps/web/public/js/site.js"
echo "OK: vitrin senkron smoke"
