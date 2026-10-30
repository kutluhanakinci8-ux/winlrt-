#!/usr/bin/env bash
# main merge sonrası hızlı yerel doğrulama.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== mixed fixture =="
bash scripts/prepare-all-benchmark-fixtures.sh

echo "== cargo test =="
cargo test -p lertiva-archive

echo "== bench KPI (Balanced, uyarı modu) =="
cargo run -p lertiva-archive --release -- bench --all --json > /tmp/winlrt-bench-balanced.json
command -v jq >/dev/null || { echo "jq yok — KPI özeti atlandı"; exit 0; }
chmod +x scripts/evaluate-bench-kpi.sh
./scripts/evaluate-bench-kpi.sh /tmp/winlrt-bench-balanced.json

echo "OK — post-merge smoke tamam."
