#!/usr/bin/env bash
# Gap plan G1–G5 repo doğrulama (G6+ henüz tanımsız strict kapı yok).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

for n in 1 2 3 4 5; do
  bash "$ROOT/scripts/verify-faz-g${n}.sh"
done
echo "OK: Faz G1–G5 repo kapıları"
