#!/usr/bin/env bash
# GitHub Releases / workflow → vitrin macOS dağıtım manifesti (G1 polish).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/downloads/lertiva-zip-macos-distribution.json"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

TAG="$(gh release list --repo "$REPO" --limit 20 --json tagName,name \
  --jq '[.[] | select(.tagName | test("^winlrt-zip-beta"))] | .[0].tagName // empty' 2>/dev/null || true)"
DMG_URL=""
EXE_URL=""
if [[ -n "$TAG" ]]; then
  while IFS= read -r name; do
    [[ -z "$name" ]] && continue
    url="https://github.com/${REPO}/releases/download/${TAG}/${name}"
    if [[ "$name" == *.dmg ]]; then
      DMG_URL="$url"
    elif [[ "$name" == *setup.exe ]] || [[ "$name" == *x64-setup.exe ]]; then
      EXE_URL="$url"
    fi
  done < <(gh release view "$TAG" --repo "$REPO" --json assets -q '.assets[].name' 2>/dev/null || true)
fi

jq -n \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%MZ)" \
  --arg tag "${TAG:-}" \
  --arg dmg "$DMG_URL" \
  --arg exe "$EXE_URL" \
  --arg workflow "https://github.com/${REPO}/actions/workflows/lertiva-zip-macos.yml" \
  '{
    product: "Lertiva Zip macOS",
    generated_at: $generated_at,
    release_tag: ($tag | if . == "" then null else . end),
    dmg_download_url: ($dmg | if . == "" then null else . end),
    windows_exe_on_same_tag: ($exe | if . == "" then null else . end),
    ci_workflow_url: $workflow,
    artifact_hint: "Actions run → lertiva-zip-macos-build (dmg) veya lertiva-zip-macos-bench (KPI)",
    notarize_deferred: true,
    note: "dmg GitHub Release yoksa workflow artifact kullanın (G3 notarize sonrası release)."
  }' >"$OUT"
echo "Yazildi: $OUT (tag=${TAG:-yok} dmg=${DMG_URL:-yok})"
