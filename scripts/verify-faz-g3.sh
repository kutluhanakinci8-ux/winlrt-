#!/usr/bin/env bash
# Faz G3 — macOS build + resmi KPI (W14 + W34).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-w14.sh"
bash "$ROOT/scripts/verify-madde-w34.sh"
echo "OK: Faz G3 — macOS CI + resmi zip_kpi vitrin (notarize APPLE_* ayrı)"
