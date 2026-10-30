#!/usr/bin/env bash
# Windows CI artifact JSON → vitrin samples (jq gerekir).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
BALANCED="${1:?bench-report-windows.json yolu}"
FAST="${2:?bench-report-windows-fast.json yolu}"

command -v jq >/dev/null || { echo "jq gerekli"; exit 1; }

for f in "$BALANCED" "$FAST"; do
  if [[ ! -f "$f" ]]; then
    echo "Dosya yok: $f"
    exit 1
  fi
  if ! jq -e '.kpi' "$f" >/dev/null; then
    echo "Geçersiz bench JSON (kpi yok): $f"
    exit 1
  fi
done

mkdir -p "$OUT"
jq '{profile, platform, generated_at, source: "windows-official-ci", disclaimer: "Resmi Faz 3 kapı — bench-report-windows artifact.", kpi}' \
  "$BALANCED" >"$OUT/windows-balanced-kpi.json"
jq '{profile, platform, generated_at, source: "windows-official-ci", disclaimer: "Resmi hız kapısı (Fast profil).", kpi}' \
  "$FAST" >"$OUT/windows-fast-kpi.json"

echo "Yazıldı: $OUT/windows-balanced-kpi.json"
echo "Yazıldı: $OUT/windows-fast-kpi.json"
echo "Sonra: git add apps/web/public/benchmark/samples && vitrin deploy"
