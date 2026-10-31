#!/usr/bin/env bash
# Faz G4 — masaüstü repo olgunluk (saha signoff şablonları + build).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_G4_DESKTOP_MATURITY.md"
bash "$ROOT/scripts/verify-dalga3-prereqs.sh"

grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md" || {
  echo "Kurumsal signoff PASS değil" >&2
  exit 1
}
grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" || {
  echo "Explorer signoff PASS değil" >&2
  exit 1
}

cargo check -p lertiva-zip --quiet
cargo check -p lertiva-arc --quiet
bash "$ROOT/scripts/check-v1-gate-ready.sh"
bash "$ROOT/scripts/evaluate-lertiva-desktop-maturity-gate.sh" >/dev/null

echo "OK: Faz G4 — repo masaüstü kapıları (saha oturumu ayrı)"
