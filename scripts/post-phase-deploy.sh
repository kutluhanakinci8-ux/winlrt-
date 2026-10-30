#!/usr/bin/env bash
# Yol haritası faz sonrası: vitrin build + VPS rsync (VPS_* ortam değişkenleri).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
exec bash "$ROOT/scripts/agent-deploy-vitrin.sh"
