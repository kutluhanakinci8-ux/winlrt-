#!/usr/bin/env bash
# Madde H: imza hattı hazır + son beta build success (imzasız OK).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

test -f "$ROOT/.github/workflows/lertiva-arc-windows.yml"
grep -q TAURI_SIGNING_PRIVATE_KEY "$ROOT/.github/workflows/lertiva-arc-windows.yml"
test -f "$ROOT/scripts/verify-windows-signature.ps1"
test -f "$ROOT/docs/WINLRTRAR_CODE_SIGNING.md"
test -f "$ROOT/docs/qa/V1_0_0_UNSIGNED_RELEASE.md"

RUN="$(gh run list --workflow=lertiva-arc-windows.yml --repo "$REPO" --branch main --limit 15 --json databaseId,conclusion,event \
  | jq -r '[.[] | select(.event=="push" and .conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true)"
if [[ -z "$RUN" ]]; then
  RUN="$(gh run list --workflow=winlrt-rar-windows.yml --repo "$REPO" --branch main --limit 15 --json databaseId,conclusion,event \
    | jq -r '[.[] | select(.event=="push" and .conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true)"
fi
test -n "$RUN"

gh release view v1.0.0 --repo "$REPO" >/dev/null

echo "OK: Madde H — imza hattı + imzasız v1.0.0 (beta workflow run=$RUN)"
