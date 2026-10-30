#!/usr/bin/env bash
# v1.0 tag öncesi: iki signoff PASS mi? (exit 0 = evet)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" || {
  echo "Explorer signoff PASS değil" >&2
  exit 1
}
grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md" || {
  echo "Kurumsal signoff PASS değil" >&2
  exit 1
}

echo "OK: v1.0 signoff kapıları PASS (imza/KPI ayrı kontrol — bkz. DALGA4_V1_OTURUM.md)"
