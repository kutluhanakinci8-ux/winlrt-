#!/usr/bin/env bash
# Dalga 4 / v1.0 — signoff durumu (bilgi; fail etmez).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== v1.0 kapı durumu ==="
for f in docs/qa/EXPLORER_QA_SIGNOFF.md docs/qa/KURUMSAL_PILOT_SIGNOFF.md; do
  echo "-- $f"
  grep -E '^\*\*Durum:\*\*|PASS|FAIL' "$ROOT/$f" || true
done
echo "-- imza (GitHub secret): TAURI_SIGNING_* — panelden kontrol"
echo "-- KPI strict: WINLRT_KPI_STRICT repo variable (isteğe bağlı)"
