#!/usr/bin/env bash
# G10 — marka + suite repo kapı özeti.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-brand-suite-gate.json}"

brand_ok=false
suite_roadmap=false
product_roadmap=false
tur4_doc=false
faz3_kickoff=false

bash "$ROOT/scripts/verify-lertiva-brand.sh" >/dev/null 2>&1 && brand_ok=true
[[ -f "$ROOT/docs/SUITE_ROADMAP.md" ]] && suite_roadmap=true
[[ -f "$ROOT/docs/PRODUCT_ROADMAP.md" ]] && product_roadmap=true
[[ -f "$ROOT/docs/qa/LERTIVA_RELEASE_POLICY_TUR4.md" ]] && tur4_doc=true
[[ -f "$ROOT/docs/qa/SUITE_FAZ3_DOCUMENT_KICKOFF.md" ]] && faz3_kickoff=true

all_pass=false
if [[ "$brand_ok" == true && "$suite_roadmap" == true && "$product_roadmap" == true && "$tur4_doc" == true && "$faz3_kickoff" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson brand "$([[ "$brand_ok" == true ]] && echo true || echo false)" \
  --argjson suite "$([[ "$suite_roadmap" == true ]] && echo true || echo false)" \
  --argjson product "$([[ "$product_roadmap" == true ]] && echo true || echo false)" \
  --argjson tur4 "$([[ "$tur4_doc" == true ]] && echo true || echo false)" \
  --argjson faz3 "$([[ "$faz3_kickoff" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva suite",
    generated_at: $generated_at,
    disclaimer: "G10 repo: Tur4 release politikası + suite kickoff; PDF modülü kod yok.",
    gates: {
      lertiva_brand_scan: { pass: $brand },
      suite_roadmap: { pass: $suite },
      product_roadmap: { pass: $product },
      tur4_release_policy: { pass: $tur4 },
      document_suite_kickoff: { pass: $faz3 }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G10 marka — brand: $brand_ok · suite: $suite_roadmap · Tur4: $tur4_doc · tümü: $all_pass"
echo "Yazıldı: $OUT"
