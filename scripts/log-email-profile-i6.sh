#!/usr/bin/env bash
# I6 rapor: mixed fixture + email profili → boyut + süre (JSON satırı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
DATA="$ROOT/benchmarks/fixtures/mixed/data"
OUT="$(mktemp --suffix=.zip)"
START=$(date +%s%3N)
cargo run -p winlrt-archive --release --bin winlrt-zip -- create "$DATA" "$OUT" --profile email >/dev/null
END=$(date +%s%3N)
MS=$((END - START))
SIZE=$(stat -c%s "$OUT" 2>/dev/null || stat -f%z "$OUT")
CAP=$((10 * 1024 * 1024))
OK=$([ "$SIZE" -le "$CAP" ] && echo true || echo false)
printf '{"scenario":"I6","profile":"email","zip_bytes":%s,"create_ms":%s,"max_bytes":%s,"size_ok":%s}\n' \
  "$SIZE" "$MS" "$CAP" "$OK"
rm -f "$OUT"
