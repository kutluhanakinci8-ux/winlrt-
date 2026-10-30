#!/usr/bin/env bash
# Post-v1 sıra: M prep → N KPI → O beta (sırayla).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/verify-madde-m.sh"
bash "$ROOT/scripts/agent-madde-n-complete.sh"
bash "$ROOT/scripts/agent-madde-d-complete.sh"
bash "$ROOT/scripts/verify-madde-i.sh"
echo "OK: Post-v1 Dalga 5 (M prep + N + O)"
