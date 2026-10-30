#!/usr/bin/env bash
# Madde W12: winlrtZIP Explorer I1/I2 (proxy interop + Windows QA scriptleri).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

grep -q WinlrtZipOpen "$ROOT/scripts/windows/winlrtzip-explorer-qa-preflight.ps1"
test -x "$ROOT/scripts/agent-winlrtzip-explorer-i1-i2-signoff.sh" || test -f "$ROOT/scripts/agent-winlrtzip-explorer-i1-i2-signoff.sh"
cargo test -p winlrt-archive --test winlrtzip_interop i1_ --quiet
cargo test -p winlrt-archive --test winlrtzip_interop i2_ --quiet
echo "OK: Madde W12 — winlrtZIP Explorer I1/I2 (proxy)"
