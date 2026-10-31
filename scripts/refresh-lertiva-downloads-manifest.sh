#!/usr/bin/env bash
# Vitrin indirme merkezi — kullanıcıya Lertiva adları (GitHub tag gizli alan).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/downloads/lertiva-downloads-manifest.json"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

bash "$ROOT/scripts/refresh-lertiva-zip-macos-distribution.sh" 2>/dev/null || true
MACOS="$(cat "$ROOT/apps/web/public/downloads/lertiva-zip-macos-distribution.json" 2>/dev/null || echo '{}')"

zip_rel="$(gh release list --repo "$REPO" --limit 25 --json tagName,name \
  --jq '[.[] | select(.tagName | test("^winlrt-zip-beta|^lertiva-zip-beta"))][0] // empty' 2>/dev/null || echo empty)"
if [[ "$zip_rel" == "empty" || -z "$zip_rel" ]]; then
  zip_rel=""
fi
arc_rel="$(gh release list --repo "$REPO" --limit 20 --json tagName,name,isPrerelease \
  --jq '[.[] | select(.tagName | test("^v1\\.0\\.[0-9]+$")) | select(.isPrerelease == false)][0] // empty' 2>/dev/null || echo empty)"

zip_tag="" zip_name="" zip_exe=""
if [[ "$zip_rel" != "empty" && -n "$zip_rel" ]]; then
  zip_tag="$(echo "$zip_rel" | jq -r '.tagName // empty')"
  zip_name="$(echo "$zip_rel" | jq -r '.name // empty')"
  if [[ -n "$zip_tag" ]]; then
    zip_exe="$(gh release view "$zip_tag" --repo "$REPO" --json assets -q '.assets[].name' 2>/dev/null | grep -i setup.exe | head -1 || true)"
  fi
fi

arc_tag="v1.0.1"
arc_name="Lertiva Arc v1.0.1"
if [[ "$arc_rel" != "empty" && -n "$arc_rel" ]]; then
  arc_tag="$(echo "$arc_rel" | jq -r '.tagName // "v1.0.1"')"
  arc_name="$(echo "$arc_rel" | jq -r '.name // "Lertiva Arc v1.0.1"')"
fi
[[ "$arc_name" == *winlrt* ]] && arc_name="Lertiva Arc ${arc_tag#v}"

zip_label="$zip_name"
if [[ -z "$zip_label" || "$zip_label" == *winlrt* || "$zip_label" == *ZIP* && "$zip_label" == winlrt* ]]; then
  zip_label="Lertiva Zip Beta"
fi
if [[ "$zip_tag" =~ beta\.([0-9]+)$ ]]; then
  zip_label="Lertiva Zip Beta ${BASH_REMATCH[1]}"
fi
[[ -z "$zip_label" ]] && zip_label="Lertiva Zip Beta"

zip_exe_url=""
[[ -n "$zip_tag" && -n "$zip_exe" ]] && zip_exe_url="https://github.com/${REPO}/releases/download/${zip_tag}/${zip_exe// /%20}"

arc_exe=""
if [[ -n "$arc_tag" ]]; then
  arc_exe="$(gh release view "$arc_tag" --repo "$REPO" --json assets -q '.assets[].name' 2>/dev/null | grep -iE 'setup\.exe$' | head -1 || true)"
fi
arc_ver="${arc_tag#v}"
[[ -z "$arc_exe" ]] && arc_exe="Lertiva Arc_${arc_ver}_x64-setup.exe"
arc_exe_url="https://github.com/${REPO}/releases/download/${arc_tag}/${arc_exe// /%20}"

jq -n \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%MZ)" \
  --arg zip_label "$zip_label" \
  --arg zip_tag "$zip_tag" \
  --arg zip_exe_url "$zip_exe_url" \
  --arg zip_release_url "https://github.com/${REPO}/releases/tag/${zip_tag}" \
  --arg arc_label "$arc_name" \
  --arg arc_tag "$arc_tag" \
  --arg arc_release_url "https://github.com/${REPO}/releases/tag/${arc_tag}" \
  --arg arc_exe_url "$arc_exe_url" \
  --argjson macos "$MACOS" \
  '{
    generated_at: $generated_at,
    brand: "Lertiva",
    products: {
      zip: {
        name: "Lertiva Zip",
        channel: "beta",
        display_label: $zip_label,
        windows: {
          installer_url: ($zip_exe_url | if . == "" then null else . end),
          release_page: (if $zip_tag == "" then null else $zip_release_url end)
        },
        macos: $macos,
        corporate_gpo: "/downloads/lertiva-zip-corporate-gpo.zip"
      },
      arc: {
        name: "Lertiva Arc",
        channel: "stable",
        display_label: $arc_label,
        windows: {
          installer_url: $arc_exe_url,
          release_page: $arc_release_url
        },
        corporate_gpo: "/downloads/lertiva-arc-corporate-gpo.zip"
      }
    }
  }' >"$OUT"
echo "Yazildi: $OUT"
