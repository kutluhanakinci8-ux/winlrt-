#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_G10_BRAND_SUITE.md"
test -f "$ROOT/docs/qa/LERTIVA_RELEASE_POLICY_TUR4.md"
bash "$ROOT/scripts/evaluate-lertiva-brand-suite-gate.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-brand-suite-gate.json" >/dev/null

echo "OK: Madde G10 — marka + suite (repo)"
