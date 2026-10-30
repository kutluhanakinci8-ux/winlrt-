#!/usr/bin/env bash
# Madde W13: Lertiva Zip Faz 2 KOBİ pilot kapisi (proxy + Windows scriptleri).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-lertiva-zip-kobi-prereqs.sh"
cargo test -p lertiva-archive --test lertiva_zip_interop kobi_ --quiet
echo "OK: Madde W13 — Lertiva Zip KOBİ pilot (proxy)"
