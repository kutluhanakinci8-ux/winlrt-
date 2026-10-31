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

echo "=== Lertiva Arc dalga sırası ==="
if git -C "$ROOT" rev-parse v1.0.1 >/dev/null 2>&1; then
  echo "→ TAMAM: v1.0.1 (Lertiva Arc installer) — https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/v1.0.1"
elif git -C "$ROOT" rev-parse v1.0.0 >/dev/null 2>&1; then
  echo "→ TAMAM: v1.0.0 (legacy winlrtRAR adı) — https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/v1.0.0"
  echo "   İsteğe bağlı: TAURI_SIGNING_* → imzalı v1.0.1"
elif ! explorer_pass; then
  echo "→ AKTİF: Dalga 2 — Windows Explorer QA"
elif ! corp_pass; then
  echo "→ AKTİF: Dalga 3 — Kurumsal pilot"
else
  echo "→ AKTİF: Dalga 4 — v1.0"
  bash "$ROOT/scripts/check-v1-gate-status.sh" | tail -n +2
fi
