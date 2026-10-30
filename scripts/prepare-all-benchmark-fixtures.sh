#!/usr/bin/env bash
# Tüm bench fixture hazırlığı (şu an: mixed ~10 MiB).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/benchmarks/fixtures/mixed/prepare.sh"
echo "Benchmark fixture hazır."
