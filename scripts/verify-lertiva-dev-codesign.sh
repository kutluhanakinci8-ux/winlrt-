#!/usr/bin/env bash
# Geliştirme kod imzası: public .cer + scriptler; private PFX repoda olmamalı.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

test -f "$ROOT/.github/signing/lertiva-dev-codesign.cer"
test -f "$ROOT/.github/signing/README.md"
test -x "$ROOT/scripts/generate-lertiva-dev-codesign.sh"
test -f "$ROOT/scripts/windows/new-lertiva-dev-codesign-cert.ps1"
grep -q 'self-signed\|Self-signed\|geliştirme' "$ROOT/docs/WINLRTRAR_CODE_SIGNING.md"
grep -q TAURI_SIGNING_PRIVATE_KEY "$ROOT/.github/workflows/lertiva-arc-windows.yml"
grep -q TAURI_SIGNING_PRIVATE_KEY "$ROOT/.github/workflows/lertiva-zip-windows.yml"
! git -C "$ROOT" ls-files --error-unmatch .signing-local 2>/dev/null
echo "OK: Lertiva dev kod imzası (self-signed) altyapısı"
