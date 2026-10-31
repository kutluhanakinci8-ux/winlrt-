#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/evaluate-lertiva-arc-platform-gate.sh" >/dev/null
jq -e '.phases["AP-3"].prep == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-arc-platform-gate.json" >/dev/null
echo "OK: Arc platform AP-3 prep"
