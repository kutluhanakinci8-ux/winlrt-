#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo test -p winlrt-archive --quiet
grep -q prepare_extract_dirs "$ROOT/crates/winlrt-archive/src/wlrt.rs"
grep -q 'for _ in 0..5' "$ROOT/crates/winlrt-archive/src/bench.rs"

echo "OK: Madde Q2 perf kodu + testler"
