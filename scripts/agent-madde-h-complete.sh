#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/verify-madde-h.sh"
echo "Imzalı v1.0.1 için: bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/cert.pfx (panel oturumu)"
