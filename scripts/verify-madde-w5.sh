#!/usr/bin/env bash
# Madde W5: winlrtZIP password UI + Explorer --secure.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/apps/winlrt-zip/src-tauri/windows/installer-hooks.nsh"
grep -q WinlrtZipCompressSecure "$HOOK"
grep -q '\-\-secure' "$HOOK"
grep -q evaluate_create_password "$ROOT/apps/winlrt-zip/src-tauri/src/lib.rs"
grep -q startup_secure_wizard "$ROOT/apps/winlrt-zip/src/App.tsx"
cargo check -p winlrt-zip --quiet
echo "OK: Madde W5 — winlrtZIP parola UI + Explorer --secure"
