#!/usr/bin/env bash
# macOS CI bench JSON → Lertiva Zip vitrin KPI örnekleri.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples"
BALANCED="${1:?macos balanced bench json}"
FAST="${2:?macos fast bench json}"

command -v jq >/dev/null || { echo "jq gerekli"; exit 1; }

for f in "$BALANCED" "$FAST"; do
  [[ -f "$f" ]] || { echo "Dosya yok: $f" >&2; exit 1; }
  jq -e '.zip_kpi' "$f" >/dev/null || { echo "zip_kpi yok: $f" >&2; exit 1; }
done

mkdir -p "$OUT"
B_JSON="$(cat "$BALANCED")"
F_JSON="$(cat "$FAST")"

jq -n --argjson data "$B_JSON" \
  '{
    product: "Lertiva Zip",
    profile: $data.profile,
    platform: $data.platform,
    generated_at: $data.generated_at,
    source: "macos-official-ci",
    disclaimer: "Resmi macOS Balanced — lertiva-zip-macos workflow.",
    kpi: $data.zip_kpi
  }' >"$OUT/lertiva-zip-macos-balanced-kpi.json"

jq -n --argjson data "$F_JSON" \
  '{
    product: "Lertiva Zip",
    profile: $data.profile,
    platform: $data.platform,
    generated_at: $data.generated_at,
    source: "macos-official-ci",
    disclaimer: "Resmi macOS Fast — lertiva-zip-macos workflow.",
    kpi: $data.zip_kpi
  }' >"$OUT/lertiva-zip-macos-fast-kpi.json"

echo "Yazıldı: lertiva-zip-macos-{balanced,fast}-kpi.json"
