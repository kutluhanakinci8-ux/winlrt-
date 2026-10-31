#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash "$ROOT/scripts/verify-madde-g10.sh"
echo "OK: Faz G10 — marka + suite repo (PDF/imza kod sprinti ayrı)"
