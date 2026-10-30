#!/usr/bin/env bash
# Madde W19: Lertiva Zip kurumsal pilot signoff kapisi.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-w18.sh"
test -f "$ROOT/scripts/agent-lertiva-zip-corporate-signoff.sh"
test -f "$ROOT/docs/qa/MADDE_W19_WINLRTZIP_CORPORATE_SIGNOFF.md"
grep -q enterprise_policy_info "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"
cargo test -p lertiva-archive audit:: --quiet

echo "OK: Madde W19 — Lertiva Zip kurumsal pilot signoff kapisi"
