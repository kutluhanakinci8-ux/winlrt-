#!/usr/bin/env bash
# W14: macOS CI tetikleyici flag güncelle → push main.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FLAG="$ROOT/.github/flags/trigger-zip-macos-build"
TS="$(date -u +%Y-%m-%dT%H:%MZ)"
{
  echo "# push: Lertiva Zip macOS beta build (Madde W14)"
  echo "# agent: bash scripts/agent-madde-w14-macos-trigger.sh"
  echo "# trigger ${TS}"
} >"$FLAG"
echo "OK: $FLAG güncellendi — commit + push ile lertiva-zip-macos.yml calisir"
