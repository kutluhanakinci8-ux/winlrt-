#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash "$ROOT/scripts/verify-faz-g1.sh"
bash "$ROOT/scripts/refresh-lertiva-zip-macos-distribution.sh" || true
bash "$ROOT/scripts/evaluate-lertiva-g1-platform-gate.sh"
echo "OK: G1 platform polish smoke"
