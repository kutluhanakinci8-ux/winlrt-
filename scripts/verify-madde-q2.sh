#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo test -p lertiva-archive --quiet
grep -q prepare_extract_dirs "$ROOT/crates/lertiva-archive/src/wlrt.rs"
grep -q 'for _ in 0..5' "$ROOT/crates/lertiva-archive/src/bench.rs"
grep -q EXTRACT_PARALLEL_MIN_RAW_BYTES "$ROOT/crates/lertiva-archive/src/wlrt.rs"
grep -q '"speed_gate": true' "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json"

echo "OK: Madde Q2 — perf + vitrin speed_gate true"
