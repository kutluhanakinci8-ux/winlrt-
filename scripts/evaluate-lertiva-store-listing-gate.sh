#!/usr/bin/env bash
# G7 — mağaza listing repo kapı özeti (gerçek URL insan doldurur).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-listing-gate.json}"

w27_ok=false
w32_ok=false
data_safety=false
listing_url=""

bash "$ROOT/scripts/verify-madde-w27.sh" >/dev/null 2>&1 && w27_ok=true
bash "$ROOT/scripts/verify-madde-w32.sh" >/dev/null 2>&1 && w32_ok=true
[[ -f "$ROOT/docs/qa/WINLRTZIP_STORE_DATA_SAFETY.md" ]] && data_safety=true

if [[ -f "$ROOT/apps/web/public/benchmark/samples/lertiva-store-listing-url.txt" ]]; then
  listing_url="$(tr -d '\r\n' <"$ROOT/apps/web/public/benchmark/samples/lertiva-store-listing-url.txt")"
fi

listing_submitted=false
[[ -n "$listing_url" ]] && listing_submitted=true

all_pass=false
if [[ "$w27_ok" == true && "$w32_ok" == true && "$data_safety" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --arg listing_url "$listing_url" \
  --argjson w27 "$([[ "$w27_ok" == true ]] && echo true || echo false)" \
  --argjson w32 "$([[ "$w32_ok" == true ]] && echo true || echo false)" \
  --argjson data_safety "$([[ "$data_safety" == true ]] && echo true || echo false)" \
  --argjson listing_submitted "$([[ "$listing_submitted" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Zip",
    generated_at: $generated_at,
    disclaimer: "G7 repo: W27+W32+Data safety; Play/App URL vitrin dosyasi ile tamamlanir.",
    gates: {
      w27_store_gate: { pass: $w27 },
      w32_store_pilot: { pass: $w32 },
      data_safety_doc: { pass: $data_safety },
      listing_url_set: { pass: $listing_submitted, url: (if $listing_url == "" then null else $listing_url end) }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G7 mağaza — W27: $w27_ok · W32: $w32_ok · data safety: $data_safety · URL: ${listing_url:-—}"
echo "Yazıldı: $OUT"
