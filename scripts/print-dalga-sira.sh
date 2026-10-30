#!/usr/bin/env bash
# Aktif dalga özeti (agent / deploy log).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

explorer_pass() {
  grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" 2>/dev/null
}

corp_pass() {
  grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md" 2>/dev/null
}

echo "=== winlrtRAR dalga sırası ==="
if ! explorer_pass; then
  echo "→ AKTİF: Dalga 2 — Windows Explorer QA"
  echo "   Chat: QA bitti + SHA256 (bkz. docs/qa/PILOT_CHAT_SABLONLARI.md)"
elif ! corp_pass; then
  echo "→ AKTİF: Dalga 3 — Kurumsal pilot"
  echo "   ZIP: https://winlrt.com.tr/downloads/winlrtRAR-corporate-gpo.zip"
else
  echo "→ AKTİF: Dalga 4 — v1.0 kapı / tag"
  bash "$ROOT/scripts/check-v1-gate-status.sh" | tail -n +2
fi
