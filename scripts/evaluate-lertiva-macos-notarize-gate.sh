#!/usr/bin/env bash
# G3 ürün — macOS notarize (flag + macOS CI yeşil).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json}"
FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

macos_ci=false
bash "$ROOT/scripts/verify-madde-w14.sh" >/dev/null 2>&1 && macos_ci=true

notarize_flag=false
[[ -f "$FLAG" ]] && grep -q 'notarize' "$FLAG" && notarize_flag=true

macos_kpi=false
bash "$ROOT/scripts/verify-madde-w34.sh" >/dev/null 2>&1 && macos_kpi=true

all_pass=false
[[ "$macos_ci" == true && "$macos_kpi" == true && "$notarize_flag" == true ]] && all_pass=true

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --arg repo "$REPO" \
  --argjson macos_ci "$([[ "$macos_ci" == true ]] && echo true || echo false)" \
  --argjson macos_kpi "$([[ "$macos_kpi" == true ]] && echo true || echo false)" \
  --argjson notarize "$([[ "$notarize_flag" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Zip macOS",
    generated_at: $generated_at,
    disclaimer: "Notarize: APPLE_* secret sonrasi flag dosyasi.",
    gates: {
      macos_build_w14: { pass: $macos_ci },
      macos_kpi_w34: { pass: $macos_kpi },
      apple_notarize_configured: { pass: $notarize, flag: "lertiva-apple-notarize-configured.flag" }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G3 notarize — W14: $macos_ci · W34: $macos_kpi · flag: $notarize_flag · tümü: $all_pass"
echo "Yazıldı: $OUT"
