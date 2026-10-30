#!/usr/bin/env bash
# Madde W18: Lertiva Zip kurumsal GPO + denetim.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-winlrtzip-corporate-prereqs.sh"
cargo test -p lertiva-archive audit:: --quiet
cargo check -p lertiva-zip --quiet
grep -q Lertiva Zip-corporate-gpo "$ROOT/apps/web/public/downloads/index.html" || \
  test -f "$ROOT/apps/web/public/downloads/Lertiva Zip-corporate-gpo.zip"

echo "OK: Madde W18 — Lertiva Zip kurumsal GPO + audit"
