#!/usr/bin/env bash
# Faz G2 — Windows Authenticode CI (dev); vitrin SmartScreen PNG opsiyonel.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-h.sh"
bash "$ROOT/scripts/verify-madde-h-signed.sh"
test -f "$ROOT/docs/qa/MADDE_H_IMZA_SIGNED.md"

jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json" >/dev/null

bash "$ROOT/scripts/verify-madde-g2-vitrin.sh"

echo "OK: Faz G2 — imza CI %100 (dev); EV/OV + gerçek vitrin PNG ayrı"
