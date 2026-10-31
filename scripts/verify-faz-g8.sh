#!/usr/bin/env bash
# G8 — Lerta staging repo (prod bearer ayrı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/run-lerta-staging-smoke.sh"
test -f "$ROOT/docs/qa/MADDE_W31_WINLRTZIP_LERTA_INTEGRATION.md"
test -f "$ROOT/docs/qa/G8_LERTA_STAGING_RUNBOOK.md"
jq -e '.repo_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-lerta-staging-gate.json" >/dev/null

echo "::warning::G8: LERTA_API_BEARER prod + live_submit IT onayı gerekir (all_pass)"
echo "OK: Faz G8 — Lerta staging repo (prod smoke ayrı)"
