#!/usr/bin/env bash
# Madde W5: Lertiva Zip password UI + Explorer --secure.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/apps/lertiva-zip/src-tauri/windows/installer-hooks.nsh"
grep -q WinlrtZipCompressSecure "$HOOK"
grep -q '\-\-secure' "$HOOK"
grep -q evaluate_create_password "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"
grep -q startup_secure_wizard "$ROOT/apps/lertiva-zip/src/App.tsx"
cargo check -p lertiva-zip --quiet
echo "OK: Madde W5 — Lertiva Zip parola UI + Explorer --secure"
