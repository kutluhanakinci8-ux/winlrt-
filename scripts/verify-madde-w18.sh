#!/usr/bin/env bash
# Madde W18: winlrtZIP kurumsal GPO + denetim.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-winlrtzip-corporate-prereqs.sh"
cargo test -p winlrt-archive audit:: --quiet
cargo check -p winlrt-zip --quiet
grep -q winlrtZIP-corporate-gpo "$ROOT/apps/web/public/downloads/index.html" || \
  test -f "$ROOT/apps/web/public/downloads/winlrtZIP-corporate-gpo.zip"

echo "OK: Madde W18 — winlrtZIP kurumsal GPO + audit"
