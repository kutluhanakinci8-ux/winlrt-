#!/usr/bin/env bash
# Lertiva Arc — çoklu platform faz kapısı (AP-0 … AP-6).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json}"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
MANIFEST="$ROOT/apps/web/public/downloads/lertiva-downloads-manifest.json"
TAURI_CONF="$ROOT/apps/lertiva-arc/src-tauri/tauri.conf.json"
FLAG_EV="$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag"

command -v jq >/dev/null || { echo "jq gerekli" >&2; exit 1; }

tauri_ver="$(grep -E '"version"' "$TAURI_CONF" | head -1 | sed 's/.*"\([0-9.]*\)".*/\1/')"

ap0_manifest=false
if [[ -f "$MANIFEST" ]]; then
  jq -e '.products.arc.platform_roadmap.windows_arm64.phase == "AP-2"' "$MANIFEST" >/dev/null 2>&1 && ap0_manifest=true
fi

ap0_workflows=false
[[ -f "$ROOT/.github/workflows/lertiva-arc-windows-stable.yml" ]] \
  && [[ -f "$ROOT/.github/workflows/lertiva-arc-windows.yml" ]] \
  && ap0_workflows=true

ap0_plan=false
[[ -f "$ROOT/docs/qa/ARC_PLATFORM_FAZ_PLAN.md" ]] && ap0_plan=true

ap0_pass=false
[[ "$ap0_manifest" == true && "$ap0_workflows" == true && "$ap0_plan" == true ]] && ap0_pass=true

ap1_prep=false
[[ "$tauri_ver" == "1.0.2" ]] \
  && grep -q "Verify Authenticode" "$ROOT/.github/workflows/lertiva-arc-windows-stable.yml" \
  && [[ -f "$ROOT/docs/qa/AP1_WINDOWS_SIGNED_RUNBOOK.md" ]] \
  && [[ -x "$ROOT/scripts/agent-arc-ap1-publish.sh" ]] \
  && ap1_prep=true

ev_ov=false
[[ -f "$FLAG_EV" ]] && grep -q 'ev_ov' "$FLAG_EV" 2>/dev/null && ev_ov=true

arc_release_tag=""
arc_signed_shipped=false
# shellcheck source=lib/gh-usable.sh
source "$ROOT/scripts/lib/gh-usable.sh"
if [[ "$gh_usable" == true ]]; then
  arc_release_tag="$(gh release list --repo "$REPO" --limit 10 --json tagName,isPrerelease \
    | jq -r '[.[] | select(.tagName | test("^v1\\.0\\.[0-9]+$")) | select(.isPrerelease == false)][0].tagName // empty' 2>/dev/null || true)"
fi
if [[ "$ev_ov" == true && "$arc_release_tag" == "v${tauri_ver}" && "$ap1_prep" == true ]]; then
  arc_signed_shipped=true
fi

ap1_pass="$arc_signed_shipped"

ap2_prep=false
[[ -f "$ROOT/.github/workflows/lertiva-arc-windows-arm64.yml" ]] \
  && grep -q "aarch64-pc-windows-msvc" "$ROOT/.github/workflows/lertiva-arc-windows-arm64.yml" \
  && [[ -f "$ROOT/docs/qa/AP2_WINDOWS_ARM64_RUNBOOK.md" ]] \
  && ap2_prep=true

ap2_ci_ok=false
if [[ "$gh_usable" == true ]]; then
  n="$(gh run list --workflow=lertiva-arc-windows-arm64.yml --repo "$REPO" --limit 10 --json conclusion \
    | jq -r '[.[] | select(.conclusion=="success")] | length' 2>/dev/null || echo 0)"
  [[ "${n:-0}" -gt 0 ]] && ap2_ci_ok=true
fi
ap2_pass="$ap2_ci_ok"

ap3_prep=false
[[ -f "$ROOT/.github/workflows/lertiva-arc-macos.yml" ]] \
  && grep -q "universal-apple-darwin" "$ROOT/.github/workflows/lertiva-arc-macos.yml" \
  && [[ -f "$ROOT/docs/qa/AP3_MACOS_ARC_RUNBOOK.md" ]] \
  && ap3_prep=true

ap3_ci_ok=false
if [[ "$gh_usable" == true ]]; then
  n="$(gh run list --workflow=lertiva-arc-macos.yml --repo "$REPO" --limit 10 --json conclusion \
    | jq -r '[.[] | select(.conclusion=="success")] | length' 2>/dev/null || echo 0)"
  [[ "${n:-0}" -gt 0 ]] && ap3_ci_ok=true
fi
ap3_pass="$ap3_ci_ok"
ap3_notarized=false
APPLE_FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag"
[[ -f "$APPLE_FLAG" ]] && grep -q 'notarize' "$APPLE_FLAG" 2>/dev/null && ap3_notarized=true

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --arg tauri_ver "$tauri_ver" \
  --arg arc_release_tag "$arc_release_tag" \
  --argjson ap0_pass "$([[ "$ap0_pass" == true ]] && echo true || echo false)" \
  --argjson ap1_prep "$([[ "$ap1_prep" == true ]] && echo true || echo false)" \
  --argjson ap1_pass "$([[ "$ap1_pass" == true ]] && echo true || echo false)" \
  --argjson ev_ov "$([[ "$ev_ov" == true ]] && echo true || echo false)" \
  --argjson ap2_prep "$([[ "$ap2_prep" == true ]] && echo true || echo false)" \
  --argjson ap2_pass "$([[ "$ap2_pass" == true ]] && echo true || echo false)" \
  --argjson ap3_prep "$([[ "$ap3_prep" == true ]] && echo true || echo false)" \
  --argjson ap3_pass "$([[ "$ap3_pass" == true ]] && echo true || echo false)" \
  --argjson ap3_notarized "$([[ "$ap3_notarized" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Arc platforms",
    generated_at: $generated_at,
    plan_doc: "docs/qa/ARC_PLATFORM_FAZ_PLAN.md",
    tauri_version: $tauri_ver,
    latest_stable_release: (if $arc_release_tag == "" then null else $arc_release_tag end),
    phases: {
      "AP-0": {
        pass: $ap0_pass,
        note: "manifest platform_roadmap + CI pistleri + faz planı"
      },
      "AP-1": {
        pass: $ap1_pass,
        prep: $ap1_prep,
        target_tag: ("v" + $tauri_ver),
        ev_ov_flag: $ev_ov,
        note: "Imzalı stable release + EV/OV flag"
      },
      "AP-2": {
        pass: $ap2_pass,
        prep: $ap2_prep,
        workflow: "lertiva-arc-windows-arm64.yml",
        note: "Windows ARM64 NSIS — CI yeşil artifact (imza AP-1 ile)"
      },
      "AP-3": {
        pass: $ap3_pass,
        prep: $ap3_prep,
        notarized: $ap3_notarized,
        workflow: "lertiva-arc-macos.yml",
        note: "macOS universal dmg — CI yeşil artifact (notarize APPLE_* ile)"
      },
      "AP-4": { pass: false, note: "Linux AppImage — bekliyor" },
      "AP-5": { pass: false, note: "Mobil Arc — bekliyor" },
      "AP-6": { pass: false, note: "Microsoft Store — bekliyor" }
    },
    all_pass: ($ap1_pass and $ap2_pass and $ap3_pass and $ap3_notarized),
    current_milestone: (
      if $ap3_pass then "AP-3"
      elif $ap2_pass then "AP-2"
      elif $ap1_pass then "AP-2-prep"
      elif $ap2_prep then "AP-1-prep+AP-2-prep"
      elif $ap0_pass then "AP-1-prep"
      else "AP-0" end
    )
  }' >"$OUT"

echo "Arc platform — AP-0: $ap0_pass · AP-1 prep: $ap1_prep · AP-1: $ap1_pass · AP-2 prep: $ap2_prep · AP-2 CI: $ap2_pass · AP-3 prep: $ap3_prep · AP-3 CI: $ap3_pass · release: ${arc_release_tag:-—}"
echo "Yazıldı: $OUT"
