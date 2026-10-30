#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md"
bash "$ROOT/scripts/verify-dalga3-prereqs.sh"
cargo test audit:: --quiet 2>/dev/null || cargo test -p winlrt-archive audit:: --quiet

echo "OK: Madde K — kurumsal pilot signoff + Dalga 3 prereqs"
