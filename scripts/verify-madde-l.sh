#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

bash "$ROOT/scripts/check-v1-gate-ready.sh"
gh release view v1.0.0 --repo "$REPO" >/dev/null
test -f "$ROOT/docs/qa/V1_0_0_UNSIGNED_RELEASE.md"

echo "OK: Madde L — v1.0.0 kapısı (imzasız resmi)"
