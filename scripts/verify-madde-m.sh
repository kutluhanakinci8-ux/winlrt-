#!/usr/bin/env bash
# Madde M prep: imza hattı + v1.0.1 pisti dokümantasyonu (secret zorunlu değil).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

test -f "$ROOT/docs/qa/MADDE_M_V1_0_1_SIGNING.md"
test -f "$ROOT/scripts/mac-set-github-tauri-signing-secrets.sh"
grep -q TAURI_SIGNING_PRIVATE_KEY "$ROOT/.github/workflows/winlrt-rar-windows.yml"
bash "$ROOT/scripts/verify-madde-h.sh"

if gh secret list --repo "${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}" 2>/dev/null | grep -q TAURI_SIGNING_PRIVATE_KEY; then
  echo "NOT: TAURI_SIGNING_PRIVATE_KEY tanımlı — strict: bash scripts/agent-madde-m-strict.sh"
else
  echo "NOT: TAURI secret yok (beklenen) — prep OK, imzalı build için panel/Mac script"
fi

echo "OK: Madde M prep — v1.0.1 pisti hazır"
