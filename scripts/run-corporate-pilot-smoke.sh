#!/usr/bin/env bash
# G5 offline smoke — GPO paketleri + denetim testleri + gate JSON.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-faz-g5.sh"
bash "$ROOT/scripts/evaluate-lertiva-corporate-gate.sh"
echo "OK: kurumsal pilot smoke"
