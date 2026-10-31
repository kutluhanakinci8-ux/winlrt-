#!/usr/bin/env bash
# G8 — Lerta staging smoke (W28–W31, vitrin JSON, dry-run).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export WINLRT_LERTA_API_DRY_RUN=1
unset LERTA_API_BEARER 2>/dev/null || true

for w in 28 29 30 31; do
  bash "$ROOT/scripts/verify-madde-w${w}.sh"
done

bash "$ROOT/scripts/evaluate-lertiva-lerta-staging-gate.sh"
echo "OK: Lerta staging smoke (G8)"
