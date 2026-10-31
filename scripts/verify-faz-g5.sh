#!/usr/bin/env bash
# Faz G5 — kurumsal saha repo hazırlığı.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-faz-g4.sh"
test -f "$ROOT/scripts/verify-lertiva-zip-corporate-prereqs.sh"
bash "$ROOT/scripts/verify-lertiva-zip-corporate-prereqs.sh"
test -f "$ROOT/docs/qa/FAZ5_KURUMSAL_BASLANGIC.md"
test -f "$ROOT/docs/qa/G5_KURUMSAL_PILOT_RUNBOOK.md"
bash "$ROOT/scripts/evaluate-lertiva-corporate-gate.sh" >/dev/null
jq -e '.repo_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-corporate-pilot-gate.json" >/dev/null

echo "OK: Faz G5 — kurumsal repo hazır (Zip saha signoff ayrı)"
