#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -f "$ROOT/docs/DALGA1_GITHUB_VPS_SECRET.md"
grep -q VPS_SSH_PASSWORD "$ROOT/docs/DALGA1_GITHUB_VPS_SECRET.md"
echo "OK: Madde T — CI VPS secret rehberi"
