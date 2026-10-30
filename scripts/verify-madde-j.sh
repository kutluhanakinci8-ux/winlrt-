#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md"
grep -qE 'sha256=[a-f0-9]{64}|Kurulum dosyası SHA256 \| [a-f0-9]{64}' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" || \
  grep -qE '[a-f0-9]{64}' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md"

bash "$ROOT/scripts/run-dalga-smoke.sh"
cargo test -p winlrt-archive --quiet

echo "OK: Madde J — Explorer signoff PASS + smoke"
