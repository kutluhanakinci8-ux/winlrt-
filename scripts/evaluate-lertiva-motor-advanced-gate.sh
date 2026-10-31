#!/usr/bin/env bash
# G9 — Arc motor ileri repo kapı özeti (seçili MVP).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-arc-motor-advanced-gate.json}"

g1_ok=false
q2_ok=false
codec_doc=false
extract_one_api=false
zip_preview=false
solid_dedup=false

if jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" >/dev/null 2>&1; then
  g1_ok=true
fi
if bash "$ROOT/scripts/verify-madde-q2.sh" >/dev/null 2>&1; then q2_ok=true; fi
[[ -f "$ROOT/docs/qa/WINLRTRAR_CODEC_MATRIX.md" ]] && codec_doc=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/wlrt.rs" && extract_one_api=true
grep -q 'entries_preview' "$ROOT/crates/lertiva-archive/src/mobile_reader.rs" && zip_preview=true
grep -q 'Solid + deduplication' "$ROOT/docs/WINLRTRAR_PLAN.md" && solid_dedup=true

all_pass=false
if [[ "$g1_ok" == true && "$q2_ok" == true && "$codec_doc" == true && "$extract_one_api" == true && "$zip_preview" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson g1 "$([[ "$g1_ok" == true ]] && echo true || echo false)" \
  --argjson q2 "$([[ "$q2_ok" == true ]] && echo true || echo false)" \
  --argjson codec_doc "$([[ "$codec_doc" == true ]] && echo true || echo false)" \
  --argjson extract_one "$([[ "$extract_one_api" == true ]] && echo true || echo false)" \
  --argjson zip_preview "$([[ "$zip_preview" == true ]] && echo true || echo false)" \
  --argjson solid_dedup "$([[ "$solid_dedup" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Arc",
    generated_at: $generated_at,
    disclaimer: "G9 repo MVP: P0+Q2+codec matrisi+extract_one; solid/dedup R&D.",
    mvp_selected: ["extract_one", "codec_matrix", "zip_entries_preview"],
    gates: {
      g1_p0_regression: { pass: $g1, note: "lertiva-zip-p0-gate.json" },
      q2_bench_perf: { pass: $q2 },
      codec_matrix_doc: { pass: $codec_doc, path: "docs/qa/WINLRTRAR_CODEC_MATRIX.md" },
      wlrt_extract_one_api: { pass: $extract_one },
      zip_mobile_entries_preview: { pass: $zip_preview },
      solid_dedup_planned: { pass: $solid_dedup, note: "plan maddesi; implementasyon yok" }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G9 motor — G1: $g1_ok · Q2: $q2_ok · codec: $codec_doc · extract_one: $extract_one_api · tümü: $all_pass"
echo "Yazıldı: $OUT"
