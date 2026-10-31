#!/usr/bin/env bash
# G4 masaüstü olgunluk — repo MVP (arama, kısayol, önizleme, ZIP tek dosya).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples/lertiva-desktop-maturity-gate.json"

search_ui=false
shortcuts_doc=false
shortcuts_ui=false
preview_cmd=false
zip_one=false
wlrt_one=false
checklist=false

grep -q 'entryQuery' "$ROOT/apps/lertiva-arc/src/App.tsx" && search_ui=true
test -f "$ROOT/docs/WINLRTRAR_SHORTCUTS.md" && shortcuts_doc=true
grep -q 'SHORTCUTS' "$ROOT/apps/lertiva-arc/src/App.tsx" && shortcuts_ui=true
grep -q 'fn preview_entry_text' "$ROOT/apps/lertiva-arc/src-tauri/src/lib.rs" && preview_cmd=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/zip_io.rs" && zip_one=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/wlrt.rs" && wlrt_one=true
test -f "$ROOT/docs/qa/G4_EXTERNAL_USER_SESSION_CHECKLIST.md" && checklist=true

all_pass=false
if [[ "$search_ui" == true && "$shortcuts_doc" == true && "$shortcuts_ui" == true \
  && "$preview_cmd" == true && "$zip_one" == true && "$wlrt_one" == true && "$checklist" == true ]]; then
  all_pass=true
fi

jq -n \
  --argjson all "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  --argjson search "$([[ "$search_ui" == true ]] && echo true || echo false)" \
  --argjson shortcuts_doc "$([[ "$shortcuts_doc" == true ]] && echo true || echo false)" \
  --argjson shortcuts_ui "$([[ "$shortcuts_ui" == true ]] && echo true || echo false)" \
  --argjson preview "$([[ "$preview_cmd" == true ]] && echo true || echo false)" \
  --argjson zip_one "$([[ "$zip_one" == true ]] && echo true || echo false)" \
  --argjson wlrt_one "$([[ "$wlrt_one" == true ]] && echo true || echo false)" \
  --argjson checklist "$([[ "$checklist" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Arc desktop maturity (G4 repo MVP)",
    all_pass: $all,
    disclaimer: "Dış kullanıcı oturumu insan adımı; repo UX MVP bu JSON.",
    gates: {
      entry_search_ui: { pass: $search },
      shortcuts_doc: { pass: $shortcuts_doc },
      shortcuts_ui: { pass: $shortcuts_ui },
      preview_entry_text: { pass: $preview },
      zip_extract_one: { pass: $zip_one },
      wlrt_extract_one: { pass: $wlrt_one },
      external_session_checklist: { pass: $checklist }
    }
  }' >"$OUT"
echo "G4 desktop — search: $search_ui · shortcuts: $shortcuts_ui · preview: $preview_cmd · all: $all_pass"
echo "Yazildi: $OUT"
