#!/usr/bin/env bash
# Faz G3 — macOS build + resmi KPI (W14 + W34).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if bash "$ROOT/scripts/verify-madde-w14.sh" 2>/dev/null; then
  :
elif bash "$ROOT/scripts/verify-madde-w14-repo-local.sh"; then
  echo "::warning::W14 yerel repo (Rust/cargo yok — tam icin: rustup + verify-madde-w14.sh)"
else
  exit 1
fi
bash "$ROOT/scripts/verify-madde-w34.sh"
echo "OK: Faz G3 — macOS CI + resmi zip_kpi vitrin (notarize APPLE_* ayrı)"
