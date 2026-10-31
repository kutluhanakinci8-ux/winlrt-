#!/usr/bin/env bash
# G2 — Windows imza kapı özeti (CI + vitrin ekran görüntüsü).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json}"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

command -v jq >/dev/null || { echo "jq gerekli" >&2; exit 1; }

ci_zip_ok=false
ci_arc_ok=false
ci_source="skipped"
# shellcheck source=lib/gh-usable.sh
source "$ROOT/scripts/lib/gh-usable.sh"
if [[ "$gh_usable" == true ]]; then
  ci_source="github_api"
  z="$(gh run list --workflow=lertiva-zip-windows.yml --repo "$REPO" --branch main --limit 5 --json conclusion \
    | jq -r '[.[] | select(.conclusion=="success")] | length' 2>/dev/null || echo 0)"
  a="$(gh run list --workflow=lertiva-arc-windows.yml --repo "$REPO" --branch main --limit 5 --json conclusion \
    | jq -r '[.[] | select(.conclusion=="success")] | length' 2>/dev/null || echo 0)"
  [[ "${z:-0}" -gt 0 ]] && ci_zip_ok=true
  [[ "${a:-0}" -gt 0 ]] && ci_arc_ok=true
fi

pipeline_ok=false
[[ -f "$ROOT/scripts/sign-windows-nsis-bundle.ps1" ]] \
  && [[ -f "$ROOT/scripts/verify-windows-signature.ps1" ]] \
  && [[ -f "$ROOT/.github/signing/lertiva-dev-codesign.cer" ]] \
  && pipeline_ok=true

smartscreen_real=false
if [[ -f "$ROOT/apps/web/public/assets/gallery/incoming/smartscreen-lertiva.png" ]]; then
  smartscreen_real=true
elif [[ -f "$ROOT/apps/web/public/assets/gallery/smartscreen-lertiva.png" ]]; then
  sz=$(wc -c <"$ROOT/apps/web/public/assets/gallery/smartscreen-lertiva.png" | tr -d ' ')
  if [[ "$sz" -gt 50000 ]]; then smartscreen_real=true; fi
fi

all_pass=false
if [[ "$ci_zip_ok" == true && "$ci_arc_ok" == true && "$pipeline_ok" == true ]]; then
  all_pass=true
elif [[ "$ci_source" == "skipped" && "$pipeline_ok" == true && "$smartscreen_real" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson ci_zip "$([[ "$ci_zip_ok" == true ]] && echo true || echo false)" \
  --argjson ci_arc "$([[ "$ci_arc_ok" == true ]] && echo true || echo false)" \
  --argjson pipeline "$([[ "$pipeline_ok" == true ]] && echo true || echo false)" \
  --argjson smartscreen_real "$([[ "$smartscreen_real" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  --arg ci_source "$ci_source" \
  '{
    product: "Lertiva Windows",
    generated_at: $generated_at,
    disclaimer: "G2 dev Authenticode CI; SmartScreen PNG ayri kapi",
    ci_query: $ci_source,
    gates: {
      ci_zip_signed_build: { pass: $ci_zip, workflow: "lertiva-zip-windows.yml" },
      ci_arc_signed_build: { pass: $ci_arc, workflow: "lertiva-arc-windows.yml" },
      signing_pipeline: { pass: $pipeline, note: "sign-windows-nsis-bundle.ps1 + dev .cer" },
      vitrin_smartscreen_real: { pass: $smartscreen_real, note: "gallery/smartscreen-lertiva.png" }
    },
    all_pass: $all_pass,
    ev_ov_next: "Kurumsal PFX + EV/OV SmartScreen"
  }' >"$OUT"
echo "G2 imza kapı — Zip CI: $ci_zip_ok · Arc CI: $ci_arc_ok · pipeline: $pipeline_ok · vitrin PNG: $smartscreen_real · tümü: $all_pass"
echo "Yazıldı: $OUT"
