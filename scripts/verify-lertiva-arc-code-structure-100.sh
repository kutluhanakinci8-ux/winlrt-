#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo test -p lertiva-archive --quiet
cargo test -p lertiva-archive dedup --quiet
cargo test -p lertiva-archive zip_extract_one --quiet
cargo check -p lertiva-arc --quiet
bash "$ROOT/scripts/scan-brand-lertiva-arc.sh"
bash "$ROOT/scripts/evaluate-lertiva-desktop-maturity-gate.sh" >/dev/null
bash "$ROOT/scripts/evaluate-lertiva-motor-advanced-gate.sh" >/dev/null
bash "$ROOT/scripts/evaluate-lertiva-arc-code-structure-gate.sh" >/dev/null
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-arc-code-structure-gate.json" >/dev/null
echo "OK: Lertiva Arc kod yapisi %100 repo kapisi"
