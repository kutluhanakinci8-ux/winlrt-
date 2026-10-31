#!/usr/bin/env bash
# G1 polish — P0/v1 + W34 + dağıtım manifest + KPI runbook.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples/lertiva-g1-platform-polish-gate.json"

p0=false v1=false w34=false runbook=false
macos_manifest=false
import_macos_script=false
import_win_script=false
downloads_page=false

jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" >/dev/null 2>&1 && p0=true
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" >/dev/null 2>&1 && v1=true
bash "$ROOT/scripts/verify-madde-w34.sh" >/dev/null 2>&1 && w34=true
test -f "$ROOT/docs/qa/G1_BENCH_KPI_IMPORT_RUNBOOK.md" && runbook=true
test -f "$ROOT/apps/web/public/downloads/lertiva-zip-macos-distribution.json" && macos_manifest=true
test -f "$ROOT/scripts/import-macos-zip-kpi-from-run.sh" && import_macos_script=true
test -f "$ROOT/scripts/import-windows-zip-kpi-from-run.sh" && import_win_script=true
grep -q 'lertiva-zip-macos-distribution.json' "$ROOT/apps/web/public/downloads/index.html" && downloads_page=true

repo_pass=false
if [[ "$p0" == true && "$v1" == true && "$w34" == true && "$runbook" == true && "$macos_manifest" == true && "$import_macos_script" == true && "$import_win_script" == true && "$downloads_page" == true ]]; then
  repo_pass=true
fi

dmg_on_release=false
if jq -e '.dmg_download_url != null' "$ROOT/apps/web/public/downloads/lertiva-zip-macos-distribution.json" >/dev/null 2>&1; then
  dmg_on_release=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson p0 "$([[ "$p0" == true ]] && echo true || echo false)" \
  --argjson v1 "$([[ "$v1" == true ]] && echo true || echo false)" \
  --argjson w34 "$([[ "$w34" == true ]] && echo true || echo false)" \
  --argjson runbook "$([[ "$runbook" == true ]] && echo true || echo false)" \
  --argjson macos_manifest "$([[ "$macos_manifest" == true ]] && echo true || echo false)" \
  --argjson dmg_release "$([[ "$dmg_on_release" == true ]] && echo true || echo false)" \
  --argjson repo_pass "$([[ "$repo_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Zip platform (G1 polish)",
    generated_at: $generated_at,
    disclaimer: "repo_pass=KPI+manifest+runbook; dmg_release=GitHub Release .dmg (G3 ile gelir).",
    gates: {
      zip_p0: { pass: $p0 },
      zip_v1: { pass: $v1 },
      w34_macos_kpi: { pass: $w34 },
      kpi_import_runbook: { pass: $runbook },
      macos_distribution_manifest: { pass: $macos_manifest },
      dmg_on_github_release: { pass: $dmg_release, note: "optional until macOS release uploads dmg" }
    },
    repo_pass: $repo_pass,
    all_pass: $repo_pass
  }' >"$OUT"
echo "G1 polish — repo: $repo_pass · dmg release: $dmg_on_release"
echo "Yazildi: $OUT"
