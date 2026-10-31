#!/usr/bin/env bash
# G8 — Lerta prod repo hazırlığı (canlı API ayrı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-w31.sh"
test -f "$ROOT/docs/qa/MADDE_W31_WINLRTZIP_LERTA_INTEGRATION.md"

echo "::warning::G8: LERTA_API_BEARER prod + live_submit IT onayı gerekir"
echo "OK: Faz G8 — Lerta staging repo (prod smoke ayrı)"
