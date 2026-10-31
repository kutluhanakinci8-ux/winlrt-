#!/usr/bin/env bash
# G2 → G3 insan adımları — hangi madde tamam?
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EV_FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag"
APPLE_FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-apple-notarize-configured.flag"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq gerekli — Mac: brew install jq" >&2
  exit 1
fi

tick() { [[ "$1" == true ]] && echo "✅" || echo "⬜"; }

dev_ok=false
if bash "$ROOT/scripts/verify-madde-h-signed.sh" >/dev/null 2>&1; then
  dev_ok=true
elif bash "$ROOT/scripts/verify-madde-h-signing-repo-local.sh" >/dev/null 2>&1; then
  dev_ok=true
  dev_note=" (yerel repo; tam CI: gh auth login)"
else
  dev_note=""
fi

ev_ov=false
[[ -f "$EV_FLAG" ]] && grep -q 'ev_ov' "$EV_FLAG" && ev_ov=true

w14=false
w34=false
bash "$ROOT/scripts/verify-madde-w14.sh" >/dev/null 2>&1 && w14=true
bash "$ROOT/scripts/verify-madde-w34.sh" >/dev/null 2>&1 && w34=true

notarize_flag=false
[[ -f "$APPLE_FLAG" ]] && grep -q 'notarize' "$APPLE_FLAG" && notarize_flag=true

smartscreen=false
[[ -f "$ROOT/apps/web/public/assets/gallery/incoming/smartscreen-lertiva.png" ]] && smartscreen=true

echo "=== G2 EV/OV (Windows) — sıra 1 ==="
echo "$(tick "$dev_ok") A0 Dev Authenticode CI (verify-madde-h-signed)${dev_note:-}"
echo "$(tick "$smartscreen") Vitrin SmartScreen PNG (incoming/)"
echo "⬜ A1 Kurumsal EV/OV PFX (CA — insan)"
echo "⬜ A2 GitHub WINDOWS_CERTIFICATE + PASSWORD (insan)"
echo "⬜ A3 Zip/Arc workflow yeşil + verify /pa (EV PFX sonrası)"
echo "$(tick "$ev_ov") A4 lertiva-ev-ov-configured.flag (ev_ov)"

echo ""
echo "=== G3 Notarize (macOS Zip) — sıra 2 (G2 bitince) ==="
echo "$(tick "$w14") B0 macOS build repo (W14)"
echo "$(tick "$w34") B0 macOS resmi KPI (W34)"
echo "⬜ B1 Apple Developer ID Application (.p12)"
echo "⬜ B2 GitHub APPLE_* (6 secret)"
echo "⬜ B3 macOS workflow + spctl notarized dmg"
echo "$(tick "$notarize_flag") B4 lertiva-apple-notarize-configured.flag (notarize)"

echo ""
bash "$ROOT/scripts/evaluate-lertiva-ev-ov-signing-gate.sh" >/dev/null
bash "$ROOT/scripts/evaluate-lertiva-macos-notarize-gate.sh" >/dev/null
g2_all=$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json")
g3_all=$(jq -r '.all_pass' "$ROOT/apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json")
echo "Kapı JSON: G2 all_pass=$g2_all · G3 all_pass=$g3_all"
echo "Runbook: docs/qa/G2_G3_SIRALI_RUNBOOK.md"
