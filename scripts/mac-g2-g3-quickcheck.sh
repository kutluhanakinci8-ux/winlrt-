#!/usr/bin/env bash
# Mac — minimum G2/G3 kontrol (yalnizca jq + repo; gh gerekmez).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> Dizin: $ROOT"
if ! command -v jq >/dev/null 2>&1; then
  echo "jq yok — kurun: brew install jq" >&2
  exit 1
fi

bash "$ROOT/scripts/verify-madde-h-signing-repo-local.sh"
bash "$ROOT/scripts/verify-ev-ov-flag-honesty.sh"
bash "$ROOT/scripts/evaluate-lertiva-ev-ov-signing-gate.sh"
bash "$ROOT/scripts/evaluate-lertiva-macos-notarize-gate.sh"

echo ""
bash "$ROOT/scripts/print-g2-g3-product-progress.sh"

if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
  echo ""
  echo "gh hazir — tam senaryo: bash scripts/test-g2-g3-scenarios.sh"
else
  echo ""
  echo "gh yok veya giris yok — CI log testi atlandi (Big Sur icin normal)."
fi
