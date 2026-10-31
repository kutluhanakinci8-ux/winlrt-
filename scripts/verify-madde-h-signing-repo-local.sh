#!/usr/bin/env bash
# Madde H — repo imza hatti (Mac/Linux, gh olmadan). Tam CI: verify-madde-h-signed.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

test -f "$ROOT/scripts/sign-windows-nsis-bundle.ps1"
test -f "$ROOT/scripts/verify-windows-signature.ps1"
test -f "$ROOT/.github/signing/lertiva-dev-codesign.cer"
grep -q WINDOWS_CERTIFICATE "$ROOT/.github/workflows/lertiva-zip-windows.yml"
grep -q WINDOWS_CERTIFICATE "$ROOT/.github/workflows/lertiva-arc-windows.yml"
grep -q 'sign-windows-nsis-bundle.ps1' "$ROOT/.github/workflows/lertiva-zip-windows.yml"

echo "OK: Madde H — imza hattı repo (yerel; CI için: gh auth login + verify-madde-h-signed.sh)"
