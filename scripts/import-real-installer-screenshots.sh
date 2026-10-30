#!/usr/bin/env bash
# Gerçek Windows ekran görüntülerini vitrine al (opsiyonel).
# Dosya adları: installer-winlrtrar-setup.png, installer-winlrtzip-setup.png, smartscreen-winlrt.png
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
IN="$ROOT/apps/web/public/assets/gallery/incoming"
OUT="$ROOT/apps/web/public/assets/gallery"
mkdir -p "$IN"

copied=0
for name in installer-winlrtrar-setup.png installer-winlrtzip-setup.png smartscreen-winlrt.png; do
  if [ -f "$IN/$name" ]; then
    cp "$IN/$name" "$OUT/$name"
    echo "OK: incoming → $name"
    copied=$((copied + 1))
  fi
done

if [ "$copied" -eq 0 ]; then
  echo "OK: incoming/ boş — vitrin render PNG kullanılıyor"
fi
