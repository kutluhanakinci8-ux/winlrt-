#!/usr/bin/env bash
# G5 — Kurumsal pilot repo kapı özeti (Arc + Zip GPO, signoff, denetim).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples/lertiva-corporate-pilot-gate.json"

arc_pack=false
zip_pack=false
arc_signoff=false
zip_signoff_field=false
w19_repo=false
audit_tests=false
runbook=false
signoff_guide=false

bash "$ROOT/scripts/verify-dalga3-prereqs.sh" >/dev/null 2>&1 && arc_pack=true
bash "$ROOT/scripts/verify-lertiva-zip-corporate-prereqs.sh" >/dev/null 2>&1 && zip_pack=true
bash "$ROOT/scripts/verify-madde-w19.sh" >/dev/null 2>&1 && w19_repo=true
cargo test -p lertiva-archive audit:: --quiet >/dev/null 2>&1 && audit_tests=true
test -f "$ROOT/docs/qa/G5_KURUMSAL_PILOT_RUNBOOK.md" && runbook=true
test -f "$ROOT/docs/qa/G5_SAHA_SIGNOFF_REHBERI.md" && signoff_guide=true

grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md" && arc_signoff=true
grep -q '\[x\] PASS' "$ROOT/docs/qa/WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md" && zip_signoff_field=true

repo_pass=false
if [[ "$arc_pack" == true && "$zip_pack" == true && "$w19_repo" == true && "$audit_tests" == true && "$runbook" == true && "$arc_signoff" == true ]]; then
  repo_pass=true
fi

all_pass=false
if [[ "$repo_pass" == true && "$zip_signoff_field" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson arc_pack "$([[ "$arc_pack" == true ]] && echo true || echo false)" \
  --argjson zip_pack "$([[ "$zip_pack" == true ]] && echo true || echo false)" \
  --argjson arc_signoff "$([[ "$arc_signoff" == true ]] && echo true || echo false)" \
  --argjson zip_signoff_field "$([[ "$zip_signoff_field" == true ]] && echo true || echo false)" \
  --argjson w19_repo "$([[ "$w19_repo" == true ]] && echo true || echo false)" \
  --argjson audit_tests "$([[ "$audit_tests" == true ]] && echo true || echo false)" \
  --argjson runbook "$([[ "$runbook" == true ]] && echo true || echo false)" \
  --argjson signoff_guide "$([[ "$signoff_guide" == true ]] && echo true || echo false)" \
  --argjson repo_pass "$([[ "$repo_pass" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva kurumsal pilot (G5)",
    generated_at: $generated_at,
    disclaimer: "repo_pass=IT paketleri+Arc signoff; all_pass=Zip saha signoff da PASS.",
    downloads: {
      arc_gpo_zip: "/downloads/lertiva-arc-corporate-gpo.zip",
      zip_gpo_zip: "/downloads/lertiva-zip-corporate-gpo.zip"
    },
    gates: {
      arc_gpo_package: { pass: $arc_pack },
      zip_gpo_package: { pass: $zip_pack },
      w19_repo_capisi: { pass: $w19_repo },
      audit_unit_tests: { pass: $audit_tests },
      g5_runbook: { pass: $runbook, path: "docs/qa/G5_KURUMSAL_PILOT_RUNBOOK.md" },
      saha_signoff_rehberi: { pass: $signoff_guide },
      arc_pilot_signoff: { pass: $arc_signoff, doc: "KURUMSAL_PILOT_SIGNOFF.md" },
      zip_field_signoff: { pass: $zip_signoff_field, doc: "WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md" }
    },
    repo_pass: $repo_pass,
    all_pass: $all_pass
  }' >"$OUT"
echo "G5 kurumsal — repo: $repo_pass · saha (Zip): $zip_signoff_field · all: $all_pass"
echo "Yazildi: $OUT"
