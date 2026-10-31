#!/usr/bin/env bash
# Self-signed PFX → GitHub TAURI_SIGNING_* (geliştirme imzası).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
PFX="${LERTIVA_DEV_SIGNING_PFX:-$ROOT/.signing-local/lertiva-dev-codesign.pfx}"
PASS="${LERTIVA_DEV_CODESIGN_PASSWORD:-}"

if [[ ! -f "$PFX" ]]; then
  echo "PFX yok: önce LERTIVA_DEV_CODESIGN_PASSWORD=… bash scripts/generate-lertiva-dev-codesign.sh" >&2
  exit 1
fi
if [[ -z "$PASS" ]]; then
  echo "LERTIVA_DEV_CODESIGN_PASSWORD gerekli" >&2
  exit 1
fi
if ! command -v gh >/dev/null 2>&1; then
  echo "gh gerekli" >&2
  exit 1
fi

base64 -w0 <"$PFX" | gh secret set TAURI_SIGNING_PRIVATE_KEY --repo "$REPO"
printf '%s' "$PASS" | gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD --repo "$REPO"
echo "OK: TAURI_SIGNING_* (dev self-signed) → $REPO"
echo "Actions: lertiva-arc-windows / lertiva-zip-windows workflow_dispatch"
