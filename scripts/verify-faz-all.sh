#!/usr/bin/env bash
# Gap plan G1–G10 repo doğrulama.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

for n in 1 2 3 4 5 6 7 8 9 10; do
  bash "$ROOT/scripts/verify-faz-g${n}.sh"
done
echo "OK: Faz G1–G10 repo kapıları"
