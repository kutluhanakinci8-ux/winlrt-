#!/usr/bin/env bash
# Madde G9: motor ileri repo kapısı.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_G9_MOTOR_ADVANCED.md"
test -f "$ROOT/docs/qa/WINLRTRAR_CODEC_MATRIX.md"
grep -q 'pub fn extract_one' "$ROOT/crates/lertiva-archive/src/wlrt.rs"
cargo test -p lertiva-archive wlrt_extract_one --quiet
bash "$ROOT/scripts/evaluate-lertiva-motor-advanced-gate.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-arc-motor-advanced-gate.json" >/dev/null

echo "OK: Madde G9 — motor ileri (repo MVP)"
