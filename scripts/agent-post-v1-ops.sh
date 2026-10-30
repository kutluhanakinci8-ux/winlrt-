#!/usr/bin/env bash
# v1.0 sonrası agent bakım: smoke + bench tetik + deploy.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DATE="$(date -u +%Y-%m-%dT%H:%MZ)"

echo "# agent post-v1 ops ${DATE}" >>"$ROOT/.github/flags/trigger-windows-bench"
echo "# agent post-v1 ops ${DATE}" >>"$ROOT/.github/flags/trigger-beta-build"

bash "$ROOT/scripts/run-agent-completion-batch.sh"
echo "OK: bench + beta build flag güncellendi (push ile CI tetiklenir)"
