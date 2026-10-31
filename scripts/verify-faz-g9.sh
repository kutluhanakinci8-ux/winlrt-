#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash "$ROOT/scripts/verify-madde-g9.sh"
echo "OK: Faz G9 — motor ileri repo MVP (Arc UI / solid ayrı)"
