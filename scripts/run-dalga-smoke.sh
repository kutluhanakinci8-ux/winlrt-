#!/usr/bin/env bash
# Agent deploy öncesi: Dalga 2/3 repo smoke (Linux).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
chmod +x "$ROOT/scripts/verify-dalga2-prereqs.sh" \
  "$ROOT/scripts/verify-corporate-assets.sh" \
  "$ROOT/scripts/verify-dalga3-prereqs.sh"
"$ROOT/scripts/verify-dalga2-prereqs.sh"
"$ROOT/scripts/verify-corporate-assets.sh"
"$ROOT/scripts/verify-dalga3-prereqs.sh"
