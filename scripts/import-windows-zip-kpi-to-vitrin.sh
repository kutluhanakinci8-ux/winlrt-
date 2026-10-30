#!/usr/bin/env bash
# Windows CI bench JSON → Lertiva Zip vitrin + P0×3 özeti.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
BALANCED="${1:?bench-report-windows.json}"
FAST="${2:?bench-report-windows-fast.json}"

command -v jq >/dev/null || { echo "jq gerekli"; exit 1; }

extract_json() {
  local f="$1"
  if jq -e '.zip_kpi' "$f" >/dev/null 2>&1; then
    cat "$f"
    return
  fi
  local raw start
  raw="$(tr -d '\r' <"$f")"
  start="${raw%%\{*}"
  raw="${raw#"$start"}"
  raw="${raw%"${raw##*\}}"}"
  printf '%s' "$raw"
}

for f in "$BALANCED" "$FAST"; do
  [[ -f "$f" ]] || { echo "Dosya yok: $f" >&2; exit 1; }
  extract_json "$f" | jq -e '.zip_kpi' >/dev/null || {
    echo "zip_kpi yok: $f" >&2
    exit 1
  }
done

mkdir -p "$OUT"
extract_json "$BALANCED" | jq '{
  product: "Lertiva Zip",
  profile,
  platform,
  generated_at,
  source: "windows-official-ci",
  disclaimer: "Resmi Lertiva Zip Balanced — bench-report-windows (zip_kpi).",
  kpi: .zip_kpi
}' >"$OUT/winlrtzip-windows-balanced-kpi.json"

extract_json "$FAST" | jq '{
  product: "Lertiva Zip",
  profile,
  platform,
  generated_at,
  source: "windows-official-ci",
  disclaimer: "Resmi Lertiva Zip Fast hız kapısı — bench-report-windows-fast.",
  kpi: .zip_kpi
}' >"$OUT/winlrtzip-windows-fast-kpi.json"

bash "$ROOT/scripts/evaluate-winlrtzip-p0-gate.sh" "$BALANCED" "$FAST" "$OUT/winlrtzip-p0-gate.json"

echo "Yazıldı: winlrtzip-windows-{balanced,fast}-kpi.json + winlrtzip-p0-gate.json"
