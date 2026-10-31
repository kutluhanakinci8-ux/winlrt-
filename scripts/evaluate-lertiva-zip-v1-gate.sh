#!/usr/bin/env bash
# Lertiva Zip v1.0 GA kapı özeti (W33): P0×3 + mağaza/mobil pilot + imza hattı hazır.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json}"
P0="$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json"
STORE="$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-pilot-gate.json"
DEV_CER="$ROOT/.github/signing/lertiva-dev-codesign.cer"

command -v jq >/dev/null || { echo "jq gerekli" >&2; exit 1; }

for f in "$P0" "$STORE"; do
  [[ -f "$f" ]] || { echo "Dosya yok: $f" >&2; exit 1; }
done

p0_pass="$(jq -r '.all_pass' "$P0")"
store_pass="$(jq -r '.all_pass' "$STORE")"
signing_dev_cer=false
[[ -f "$DEV_CER" ]] && signing_dev_cer=true

all_pass=true
[[ "$p0_pass" == "true" ]] || all_pass=false
[[ "$store_pass" == "true" ]] || all_pass=false

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"

mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg product "Lertiva Zip" \
  --arg generated_at "$generated_at" \
  --argjson p0_pass "$([[ "$p0_pass" == "true" ]] && echo true || echo false)" \
  --argjson store_pilot_pass "$([[ "$store_pass" == "true" ]] && echo true || echo false)" \
  --argjson signing_dev_cer "$signing_dev_cer" \
  --argjson all_pass "$([[ "$all_pass" == "true" ]] && echo true || echo false)" \
  '{
    product: $product,
    version_target: "1.0.0",
    generated_at: $generated_at,
    disclaimer: "W33: P0×3 resmi Windows + W32 pilot hazırlık. EV/OV imzalı release ayrı (G2).",
    gates: {
      p0_kpi: { pass: $p0_pass, source: "lertiva-zip-p0-gate.json" },
      store_pilot: { pass: $store_pilot_pass, source: "lertiva-zip-store-pilot-gate.json" },
      signing_pipeline: {
        pass: $signing_dev_cer,
        note: "Dev self-signed .cer + TAURI_SIGNING_* (SmartScreen EV/OV değil)"
      }
    },
    vitrin_status: (if $all_pass then "v1.0.0 (kapı)" else "Beta 0.1.0" end),
    all_pass: $all_pass
  }' >"$OUT"

echo "Lertiva Zip v1 kapı — P0: $p0_pass · W32: $store_pass · imza hattı cer: $signing_dev_cer · tümü: $all_pass"
echo "Yazıldı: $OUT"
