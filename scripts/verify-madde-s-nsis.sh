#!/usr/bin/env bash
# Madde S: Explorer shell etiketleri (ASCII NSIS — CI güvenilir).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/apps/lertiva-arc/src-tauri/windows/installer-hooks.nsh"
grep -q NSIS_HOOK_POSTINSTALL "$HOOK"
grep -q WinlrtCompress "$HOOK"
echo "OK: Madde S — NSIS Explorer hook mevcut (ASCII etiketler)"
