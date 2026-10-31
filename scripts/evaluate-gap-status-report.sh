#!/usr/bin/env bash
# Gap durum ozeti → JSON (vitrin + agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-gap-status-report.json}"
PCT="$ROOT/docs/qa/gap-status-percentages.json"

bash "$ROOT/scripts/evaluate-lertiva-zip-p0-gate.sh" >/dev/null 2>&1 || true
for s in evaluate-lertiva-windows-signing-gate evaluate-lertiva-ev-ov-signing-gate \
  evaluate-lertiva-macos-notarize-gate evaluate-lertiva-store-listing-gate \
  evaluate-lertiva-mobile-mvp-gate evaluate-lertiva-motor-advanced-gate \
  evaluate-lertiva-brand-suite-gate evaluate-lertiva-desktop-maturity-gate \
  evaluate-lertiva-corporate-gate evaluate-lertiva-lerta-staging-gate \
  evaluate-lertiva-g1-platform-gate evaluate-lertiva-arc-code-structure-gate; do
  bash "$ROOT/scripts/${s}.sh" >/dev/null 2>&1 || true
done

repo_g10_ok=false
bash "$ROOT/scripts/verify-faz-all.sh" >/dev/null 2>&1 && repo_g10_ok=true

summary="$(cat "$PCT")"

jq -n \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%MZ)" \
  --argjson repo_faz_all "$([[ "$repo_g10_ok" == true ]] && echo true || echo false)" \
  --argjson p0 "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" 2>/dev/null || echo false)" \
  --argjson v1 "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" 2>/dev/null || echo false)" \
  --argjson g2_dev "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json" 2>/dev/null || echo false)" \
  --argjson g2_ev "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json" 2>/dev/null || echo false)" \
  --argjson g3 "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json" 2>/dev/null || echo false)" \
  --argjson g7_listing "$(jq -r '.gates.listing_url_set.pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-listing-gate.json" 2>/dev/null || echo false)" \
  --argjson g9 "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-arc-motor-advanced-gate.json" 2>/dev/null || echo false)" \
  --argjson g10 "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-brand-suite-gate.json" 2>/dev/null || echo false)" \
  --argjson g4_desktop "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-desktop-maturity-gate.json" 2>/dev/null || echo false)" \
  --argjson g5_corporate_repo "$(jq -r '.repo_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-corporate-pilot-gate.json" 2>/dev/null || echo false)" \
  --argjson g5_corporate_all "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-corporate-pilot-gate.json" 2>/dev/null || echo false)" \
  --argjson g8_lerta_repo "$(jq -r '.repo_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-lerta-staging-gate.json" 2>/dev/null || echo false)" \
  --argjson g8_lerta_all "$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-lerta-staging-gate.json" 2>/dev/null || echo false)" \
  --argjson g1_polish "$(jq -r '.repo_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-g1-platform-polish-gate.json" 2>/dev/null || echo false)" \
  --argjson summary "$summary" \
  '{
    product: "Lertiva suite",
    generated_at: $generated_at,
    disclaimer: "Yuzdeler docs/qa/gap-status-percentages.json + GAP_STATUS_RAPOR.md; sertifika/Play/App Store ertelenmis.",
    summary: ($summary | {layers, phases, next_sequence}),
    gates_snapshot: {
      repo_verify_faz_g1_g10: $repo_faz_all,
      zip_p0: $p0,
      zip_v1: $v1,
      g2_windows_dev_signing: $g2_dev,
      g2_ev_ov_product: $g2_ev,
      g3_notarize_product: $g3,
      g4_desktop_maturity: $g4_desktop,
      g5_corporate_repo_pass: $g5_corporate_repo,
      g5_corporate_all_pass: $g5_corporate_all,
      g8_lerta_repo_pass: $g8_lerta_repo,
      g8_lerta_all_pass: $g8_lerta_all,
      g1_platform_polish: $g1_polish,
      g7_play_listing_url: $g7_listing,
      g9_motor_mvp: $g9,
      g10_brand_repo: $g10
    },
    deferred_external: ["EV/OV PFX", "APPLE notarize", "Play internal", "App Store", "LERTA prod bearer"]
  }' >"$OUT"
echo "Yazildi: $OUT"
