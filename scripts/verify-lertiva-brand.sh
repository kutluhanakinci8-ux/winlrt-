#!/usr/bin/env bash
# Lertiva marka kalıntısı: ürün adları + vitrin dosya yolları.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FAIL=0

check_file() {
  local rel="$1"
  local file="$ROOT/$rel"
  [ -f "$file" ] || return 0
  if grep -qE 'winlrtRAR|winlrtZIP|winlrtzip-|winlrtrar-|tauri-winlrt|installer-winlrt|smartscreen-winlrt' "$file"; then
    if grep -nE 'winlrtRAR|winlrtZIP|winlrtzip-|winlrtrar-|tauri-winlrt|installer-winlrt|smartscreen-winlrt' "$file" \
      | grep -qvE 'winlrt-|winlrt\.com|winlrt_rar|winlrt_zip|rebrand-lertiva'; then
      echo "FAIL: $rel"
      FAIL=1
    fi
  fi
}

while IFS= read -r rel; do
  case "$rel" in
    apps/web/dist/*|node_modules/*|target/*|*.png|Cargo.lock) continue ;;
  esac
  check_file "$rel"
done < <(rg -l 'winlrtRAR|winlrtZIP|winlrtzip-|winlrtrar-|tauri-winlrt|installer-winlrt|smartscreen-winlrt' "$ROOT" 2>/dev/null || true)

# Vitrin public kökünde eski benchmark/galeri adı olmamalı
if rg -q 'winlrtzip-|winlrtrar-|tauri-winlrt|smartscreen-winlrt' "$ROOT/apps/web/public" 2>/dev/null; then
  echo "FAIL: apps/web/public eski dosya yolu referansı"
  rg 'winlrtzip-|winlrtrar-|tauri-winlrt|smartscreen-winlrt' "$ROOT/apps/web/public" | head -5
  FAIL=1
fi

# Tur 3: beta workflow dosya adları (release tag winlrt-* bilinçli kalabilir)
for wf in lertiva-arc-windows.yml lertiva-zip-windows.yml lertiva-zip-macos.yml; do
  if [[ ! -f "$ROOT/.github/workflows/$wf" ]]; then
    echo "FAIL: eksik workflow $wf"
    FAIL=1
  fi
done
for old in winlrt-rar-windows.yml winlrt-zip-windows.yml winlrt-zip-macos.yml; do
  if [[ -f "$ROOT/.github/workflows/$old" ]]; then
    echo "FAIL: eski workflow hâlâ var: $old"
    FAIL=1
  fi
done
grep -q 'cargo check -p lertiva-zip' "$ROOT/.github/workflows/main-pipeline.yml" || {
  echo "FAIL: main-pipeline lertiva-zip check"
  FAIL=1
}

test "$FAIL" -eq 0 || exit 1
echo "OK: Lertiva marka — ürün adı ve vitrin yolu kalıntısı yok"
