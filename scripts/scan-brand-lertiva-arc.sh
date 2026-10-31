#!/usr/bin/env bash
# Arc kabuk ve paket icinde winlrt kalintisi (kullanici gorunur).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0
while IFS= read -r hit; do
  echo "winlrt: $hit" >&2
  FAIL=1
done < <(rg -n 'winlrt' "$ROOT/apps/lertiva-arc" \
  --glob '!dist/**' --glob '!**/gen/**' --glob '!README.md' --glob '!package-lock.json' 2>/dev/null \
  | rg -v 'winlrt\.com\.tr' || true)

if [[ "$FAIL" -ne 0 ]]; then
  echo "Marka taramasi basarisiz — Lertiva Arc icinde winlrt kalintisi" >&2
  exit 1
fi
echo "OK: Lertiva Arc marka taramasi"
