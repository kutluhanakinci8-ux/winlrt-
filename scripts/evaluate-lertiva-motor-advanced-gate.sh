#!/usr/bin/env bash
# G9 — Arc motor ileri repo kapı özeti (MVP + sprint2).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-arc-motor-advanced-gate.json}"

g1_ok=false
q2_ok=false
codec_doc=false
extract_one_api=false
zip_extract_one=false
partial_extract=false
dedup_scan=false
arc_ui_extract=false
arc_partial_ui=false
zip_preview=false
solid_dedup=false

if jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" >/dev/null 2>&1; then
  g1_ok=true
fi
if bash "$ROOT/scripts/verify-madde-q2.sh" >/dev/null 2>&1; then q2_ok=true; fi
[[ -f "$ROOT/docs/qa/WINLRTRAR_CODEC_MATRIX.md" ]] && codec_doc=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/wlrt.rs" && extract_one_api=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/zip_io.rs" && zip_extract_one=true
grep -q 'pub fn extract_partial' "$ROOT/crates/lertiva-archive/src/zip_io.rs" && partial_extract=true
grep -q 'pub fn scan_duplicate_files' "$ROOT/crates/lertiva-archive/src/dedup.rs" && dedup_scan=true
grep -q 'extract_one_entry' "$ROOT/apps/lertiva-arc/src-tauri/src/lib.rs" \
  && grep -q 'Tek dosyayı çıkar' "$ROOT/apps/lertiva-arc/src/App.tsx" && arc_ui_extract=true
grep -q 'Kısmi aç' "$ROOT/apps/lertiva-arc/src/App.tsx" && arc_partial_ui=true
grep -q 'entries_preview' "$ROOT/crates/lertiva-archive/src/mobile_reader.rs" && zip_preview=true
grep -q 'Solid + deduplication' "$ROOT/docs/WINLRTRAR_PLAN.md" && solid_dedup=true

mvp_pass=false
if [[ "$g1_ok" == true && "$q2_ok" == true && "$codec_doc" == true && "$extract_one_api" == true && "$arc_ui_extract" == true && "$zip_preview" == true ]]; then
  mvp_pass=true
fi

sprint2_pass=false
if [[ "$mvp_pass" == true && "$zip_extract_one" == true && "$partial_extract" == true && "$dedup_scan" == true && "$arc_partial_ui" == true ]]; then
  sprint2_pass=true
fi

all_pass="$sprint2_pass"

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson g1 "$([[ "$g1_ok" == true ]] && echo true || echo false)" \
  --argjson q2 "$([[ "$q2_ok" == true ]] && echo true || echo false)" \
  --argjson codec_doc "$([[ "$codec_doc" == true ]] && echo true || echo false)" \
  --argjson extract_one "$([[ "$extract_one_api" == true ]] && echo true || echo false)" \
  --argjson zip_extract_one "$([[ "$zip_extract_one" == true ]] && echo true || echo false)" \
  --argjson partial_extract "$([[ "$partial_extract" == true ]] && echo true || echo false)" \
  --argjson dedup_scan "$([[ "$dedup_scan" == true ]] && echo true || echo false)" \
  --argjson arc_ui "$([[ "$arc_ui_extract" == true ]] && echo true || echo false)" \
  --argjson arc_partial_ui "$([[ "$arc_partial_ui" == true ]] && echo true || echo false)" \
  --argjson zip_preview "$([[ "$zip_preview" == true ]] && echo true || echo false)" \
  --argjson solid_dedup "$([[ "$solid_dedup" == true ]] && echo true || echo false)" \
  --argjson mvp_pass "$([[ "$mvp_pass" == true ]] && echo true || echo false)" \
  --argjson sprint2_pass "$([[ "$sprint2_pass" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Arc",
    generated_at: $generated_at,
    disclaimer: "G9 sprint2: ZIP kısmi açma + dedup tarama + MVP kapılar.",
    mvp_selected: ["extract_one", "arc_ui_single_extract", "codec_matrix", "zip_entries_preview"],
    sprint2: ["zip_extract_one", "partial_extract", "dedup_scan", "arc_partial_ui"],
    gates: {
      g1_p0_regression: { pass: $g1, note: "lertiva-zip-p0-gate.json" },
      q2_bench_perf: { pass: $q2 },
      codec_matrix_doc: { pass: $codec_doc, path: "docs/qa/WINLRTRAR_CODEC_MATRIX.md" },
      wlrt_extract_one_api: { pass: $extract_one },
      zip_extract_one_api: { pass: $zip_extract_one },
      zip_partial_extract_api: { pass: $partial_extract },
      dedup_scan_api: { pass: $dedup_scan },
      arc_ui_extract_one: { pass: $arc_ui, app: "apps/lertiva-arc" },
      arc_ui_partial_extract: { pass: $arc_partial_ui },
      zip_mobile_entries_preview: { pass: $zip_preview },
      solid_dedup_planned: { pass: $solid_dedup, note: "solid arşiv R&D devam" }
    },
    mvp_pass: $mvp_pass,
    sprint2_pass: $sprint2_pass,
    all_pass: $all_pass
  }' >"$OUT"
echo "G9 motor — sprint2: $sprint2_pass · all_pass: $all_pass"
echo "Yazıldı: $OUT"
