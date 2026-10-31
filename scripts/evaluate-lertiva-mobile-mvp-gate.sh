#!/usr/bin/env bash
# G6 / W36 — mobil MVP repo kapı özeti.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-zip-mobile-mvp-gate.json}"

w26_ok=false
if bash "$ROOT/scripts/verify-madde-w26.sh" >/dev/null 2>&1; then w26_ok=true; fi

bundle_ok=false
grep -q 'com.lertiva.zip' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json" && bundle_ok=true

poc_doc=false
[[ -f "$ROOT/apps/mobile/README.md" ]] && poc_doc=true

all_pass=false
[[ "$w26_ok" == true && "$bundle_ok" == true && "$poc_doc" == true ]] && all_pass=true

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson w26 "$([[ "$w26_ok" == true ]] && echo true || echo false)" \
  --argjson bundle "$([[ "$bundle_ok" == true ]] && echo true || echo false)" \
  --argjson poc_doc "$([[ "$poc_doc" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Zip",
    generated_at: $generated_at,
    disclaimer: "G6 repo: W26 okuyucu; native apk/ipa mağaza sprinti ayrı.",
    gates: {
      w26_mobile_reader: { pass: $w26, note: "verify-madde-w26.sh" },
      bundle_id_com_lertiva_zip: { pass: $bundle },
      mobile_poc_readme: { pass: $poc_doc, path: "apps/mobile/README.md" }
    },
    store_next: "Play internal / TestFlight (G7)",
    all_pass: $all_pass
  }' >"$OUT"
echo "G6 mobil MVP — W26: $w26_ok · bundle: $bundle_ok · poc: $poc_doc · tümü: $all_pass"
echo "Yazıldı: $OUT"
