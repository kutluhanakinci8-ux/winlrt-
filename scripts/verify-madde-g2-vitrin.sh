#!/usr/bin/env bash
# G2 vitrin: import script + (strict) gerçek SmartScreen PNG.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
STRICT="${WINLRT_G2_VITRIN_STRICT:-0}"

test -f "$ROOT/docs/qa/MADDE_G2_VITRIN_SCREENSHOTS.md"
test -f "$ROOT/scripts/import-real-installer-screenshots.sh"
test -f "$ROOT/apps/web/public/assets/gallery/incoming/README.txt"

bash "$ROOT/scripts/evaluate-lertiva-windows-signing-gate.sh"

if jq -e '.gates.vitrin_smartscreen_real.pass == true' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json" >/dev/null; then
  echo "OK: Madde G2 vitrin — gerçek SmartScreen/kurulum PNG"
  exit 0
fi

if [[ "$STRICT" == "1" ]]; then
  echo "G2 vitrin strict: incoming/smartscreen-lertiva.png gerekli" >&2
  exit 1
fi

echo "::warning::G2 vitrin PNG yok — bkz. docs/qa/MADDE_G2_VITRIN_SCREENSHOTS.md"
echo "OK: Madde G2 vitrin — altyapı (PNG bekleniyor)"
