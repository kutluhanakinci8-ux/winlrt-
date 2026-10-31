#!/usr/bin/env bash
# Lertiva Arc kod yapisi F0–F10 kapı ozeti → vitrin JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples/lertiva-arc-code-structure-gate.json"

motor_rar_one=false
motor_7z_one=false
motor_recovery=false
motor_dedup=false
arc_tree=false
arc_rar_ui=false
explorer_ps1=false
brand_scan=false
manifest_dynamic=false

grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/rar/mod.rs" && motor_rar_one=true
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/sevenz.rs" && motor_7z_one=true
test -f "$ROOT/crates/lertiva-archive/src/recovery.rs" && motor_recovery=true
grep -q 'dedup_file_list' "$ROOT/crates/lertiva-archive/src/dedup.rs" && motor_dedup=true
grep -q 'entryTree' "$ROOT/apps/lertiva-arc/src/App.tsx" && arc_tree=true
grep -q '"rar"' "$ROOT/apps/lertiva-arc/src-tauri/src/lib.rs" && arc_rar_ui=true
test -f "$ROOT/scripts/lertiva-arc-explorer-qa-preflight.ps1" && explorer_ps1=true
bash "$ROOT/scripts/scan-brand-lertiva-arc.sh" >/dev/null 2>&1 && brand_scan=true
grep -q 'gh release view.*arc_tag' "$ROOT/scripts/refresh-lertiva-downloads-manifest.sh" \
  || grep -q 'gh release view "$arc_tag"' "$ROOT/scripts/refresh-lertiva-downloads-manifest.sh" && manifest_dynamic=true

all_pass=false
if [[ "$motor_rar_one" == true && "$motor_7z_one" == true && "$motor_recovery" == true \
  && "$motor_dedup" == true && "$arc_tree" == true && "$arc_rar_ui" == true \
  && "$explorer_ps1" == true && "$brand_scan" == true && "$manifest_dynamic" == true ]]; then
  all_pass=true
fi

jq -n \
  --argjson all "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  --argjson motor_rar_one "$([[ "$motor_rar_one" == true ]] && echo true || echo false)" \
  --argjson motor_7z_one "$([[ "$motor_7z_one" == true ]] && echo true || echo false)" \
  --argjson motor_recovery "$([[ "$motor_recovery" == true ]] && echo true || echo false)" \
  --argjson motor_dedup "$([[ "$motor_dedup" == true ]] && echo true || echo false)" \
  --argjson arc_tree "$([[ "$arc_tree" == true ]] && echo true || echo false)" \
  --argjson arc_rar_ui "$([[ "$arc_rar_ui" == true ]] && echo true || echo false)" \
  --argjson explorer_ps1 "$([[ "$explorer_ps1" == true ]] && echo true || echo false)" \
  --argjson brand_scan "$([[ "$brand_scan" == true ]] && echo true || echo false)" \
  --argjson manifest_dynamic "$([[ "$manifest_dynamic" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Arc code structure (F0-F10 repo)",
    all_pass: $all,
    layers: {
      motor: { target_pct: 100, gates: { rar_extract_one: $motor_rar_one, sevenz_extract_one: $motor_7z_one, recovery_module: $motor_recovery, wlrt_dedup_create: $motor_dedup }},
      arc_shell: { target_pct: 100, gates: { entry_tree: $arc_tree, rar_extract_one_ui: $arc_rar_ui }},
      explorer: { target_pct: 100, gates: { arc_explorer_preflight: $explorer_ps1 }},
      brand: { target_pct: 100, gates: { scan_clean: $brand_scan }},
      vitrin_manifest: { target_pct: 100, gates: { dynamic_arc_exe: $manifest_dynamic }}
    }
  }' >"$OUT"
echo "Arc code structure all_pass=$all_pass → $OUT"
