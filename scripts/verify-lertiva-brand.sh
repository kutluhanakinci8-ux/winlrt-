#!/usr/bin/env bash
# Lertiva Arc / Lertiva Zip kullanıcı markası kalıntısı (allowlist ile).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

FAIL=0
while IFS= read -r line; do
  file="${line%%:*}"
  rel="${file#"$ROOT"/}"
  case "$rel" in
    apps/web/dist/*|node_modules/*|target/*|*.png|Cargo.lock) continue ;;
  esac
  if grep -qE 'Lertiva Arc|Lertiva Zip' "$file" 2>/dev/null; then
    # allow: GitHub repo path, domain, eski release tag adları
    if grep -E 'winlrtRAR|winlrtZIP' "$file" | grep -qvE 'winlrt-|winlrt\.com|winlrt_rar|winlrt_zip'; then
      echo "FAIL: $rel"
      grep -nE 'Lertiva Arc|Lertiva Zip' "$file" | head -3
      FAIL=1
    fi
  fi
done < <(rg -l 'Lertiva Arc|Lertiva Zip' "$ROOT" 2>/dev/null || true)

test "$FAIL" -eq 0 || exit 1
echo "OK: Lertiva marka — Lertiva Arc/Lertiva Zip kalıntısı yok (allowlist hariç)"
