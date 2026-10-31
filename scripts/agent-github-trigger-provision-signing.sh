#!/usr/bin/env bash
# Provision workflow tetikle (flag push veya gh workflow run).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
FLAG="$ROOT/.github/flags/trigger-provision-tauri-signing"

echo "# agent $(date -u +%Y-%m-%dT%H:%MZ)" >>"$FLAG"
git add "$FLAG"
git commit -m "chore(ci): provision TAURI signing tetik" || true
git push origin HEAD

if gh workflow run provision-tauri-signing-secrets.yml --ref "$(git branch --show-current)" 2>/dev/null; then
  echo "OK: workflow_dispatch provision-tauri-signing-secrets.yml"
else
  echo "Not: workflow_dispatch 403 — push ile workflow zaten tetiklendi."
fi
