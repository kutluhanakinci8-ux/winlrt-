#!/usr/bin/env bash
# Agent izi: tüm Linux smoke + test (pilot saha hariç).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "==> Dalga smoke"
bash scripts/run-dalga-smoke.sh

echo "==> Rust test"
cargo test -p winlrt-archive

echo "==> Vitrin build"
npm ci
npm run build:web

echo "==> v1 kapı özeti"
bash scripts/check-v1-gate-status.sh || true
if bash scripts/check-v1-gate-ready.sh 2>/dev/null; then
  echo "Signoff kapıları: PASS"
else
  echo "Signoff kapıları: henüz PASS değil (beklenen — pilot saha)"
fi

echo "OK: agent completion batch"
