#!/usr/bin/env bash
# Toplu tur sonu: doğrula + VPS deploy (tek sefer).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/run-agent-completion-batch.sh"
bash "$ROOT/scripts/post-phase-deploy.sh"
