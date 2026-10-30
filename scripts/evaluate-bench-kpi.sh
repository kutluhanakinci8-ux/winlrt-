#!/usr/bin/env bash
# Faz 3 KPI özeti — bench JSON (Linux CI / yerel; jq gerekir)
set -euo pipefail
JSON="${1:?bench JSON yolu}"
STRICT="${WINLRT_KPI_STRICT:-0}"

if [[ ! -f "$JSON" ]]; then
  echo "Dosya yok: $JSON" >&2
  exit 1
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "jq bulunamadı" >&2
  exit 1
fi

profile=$(jq -r '.profile' "$JSON")
platform=$(jq -r '.platform' "$JSON")
comp_gate=$(jq -r '.kpi.compression_gate' "$JSON")
comp_pass=$(jq -r '.kpi.compression_pass_count' "$JSON")
comp_total=$(jq -r '.kpi.compression_total' "$JSON")
speed_gate=$(jq -r '.kpi.speed_gate // empty' "$JSON")

echo "Profil: $profile"
echo "Platform: $platform"
echo "Sıkıştırma kapısı: $comp_gate ($comp_pass/$comp_total)"
if [[ -n "$speed_gate" ]]; then
  speed_pass=$(jq -r '.kpi.speed_pass_count' "$JSON")
  speed_total=$(jq -r '.kpi.speed_total' "$JSON")
  echo "Hız kapısı (Fast): $speed_gate ($speed_pass/$speed_total)"
else
  echo "Hız kapısı: (bu profilde ölçülmedi)"
fi

jq -r '.kpi.rows[] | "  - \(.fixture): sıkıştırma=\(.compression_ok)\(if .speed_ok != null then ", hız=\(.speed_ok)" else "" end)"' "$JSON"

fail=0
[[ "$comp_gate" == "true" ]] || fail=1
if [[ -n "$speed_gate" && "$speed_gate" != "true" ]]; then
  fail=1
fi

if [[ "$fail" -eq 1 ]]; then
  echo "::warning::Faz 3 KPI kapısı henüz tam geçmedi — beta için uyarı."
  if [[ "$STRICT" == "1" ]]; then
    exit 1
  fi
  exit 0
fi
echo "Faz 3 KPI kapısı (bu profil): GEÇTİ"
