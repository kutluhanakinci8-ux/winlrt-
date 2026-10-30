#!/usr/bin/env bash
# Madde W13: winlrtZIP Faz 2 KOBİ pilot kapisi (proxy + Windows scriptleri).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-winlrtzip-kobi-prereqs.sh"
cargo test -p winlrt-archive --test winlrtzip_interop kobi_ --quiet
echo "OK: Madde W13 — winlrtZIP KOBİ pilot (proxy)"
