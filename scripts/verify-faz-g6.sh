#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash "$ROOT/scripts/verify-madde-w36.sh"
echo "OK: Faz G6 — mobil MVP repo (native binary ayrı)"
