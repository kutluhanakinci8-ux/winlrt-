#!/usr/bin/env bash
# Windows CI artifact JSON → vitrin samples (jq gerekir).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
BALANCED="${1:?bench-report-windows.json yolu}"
FAST="${2:?bench-report-windows-fast.json yolu}"

command -v jq >/dev/null || { echo "jq gerekli"; exit 1; }

extract_json() {
  local f="$1"
  if jq -e '.kpi' "$f" >/dev/null 2>&1; then
    cat "$f"
    return
  fi
  local raw start end
  raw="$(tr -d '\r' <"$f")"
  start="${raw%%\{*}"
  raw="${raw#"$start"}"
  end="${raw##*\}}"
  raw="${raw%"$end"}"
  raw="${raw}\}"
  printf '%s' "$raw"
}

for f in "$BALANCED" "$FAST"; do
  if [[ ! -f "$f" ]]; then
    echo "Dosya yok: $f"
    exit 1
  fi
  if ! extract_json "$f" | jq -e '.kpi' >/dev/null; then
    echo "Geçersiz bench JSON (kpi yok): $f"
    exit 1
  fi
done

mkdir -p "$OUT"
extract_json "$BALANCED" | jq '{profile, platform, generated_at, source: "windows-official-ci", disclaimer: "Resmi Faz 3 kapı — bench-report-windows artifact.", kpi}' \
  >"$OUT/windows-balanced-kpi.json"
extract_json "$FAST" | jq '{profile, platform, generated_at, source: "windows-official-ci", disclaimer: "Resmi hız kapısı (Fast profil).", kpi}' \
  >"$OUT/windows-fast-kpi.json"

echo "Yazıldı: $OUT/windows-balanced-kpi.json"
echo "Yazıldı: $OUT/windows-fast-kpi.json"
echo "Sonra: git add apps/web/public/benchmark/samples && vitrin deploy"
