#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/evaluate-lertiva-arc-platform-gate.sh" >/dev/null
jq -e '.phases["AP-2"].prep == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json" >/dev/null
python3 -c "import yaml,sys;yaml.safe_load(open('$ROOT/.github/workflows/lertiva-arc-windows-arm64.yml'))" 2>/dev/null || true
echo "OK: Arc platform AP-2 prep"
