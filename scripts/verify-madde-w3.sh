#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

test -f "$ROOT/.github/workflows/winlrt-zip-windows.yml"

TAG="$(gh release list --repo "$REPO" --limit 40 --json tagName,isPrerelease \
  | jq -r '[.[] | select(.isPrerelease and (.tagName | startswith("winlrt-zip-beta.")))] | .[0].tagName // empty')"
test -n "$TAG"

ASSET="$(gh release view "$TAG" --repo "$REPO" --json assets \
  | jq -r '.assets[] | select(.name | endswith("-setup.exe")) | .name' | head -1)"
test -n "$ASSET"

RUN="$(gh run list --workflow=winlrt-zip-windows.yml --repo "$REPO" --branch main --limit 15 --json databaseId,conclusion,event \
  | jq -r '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty')"
test -n "$RUN"

grep -q winlrt-zip-beta "$ROOT/apps/web/public/downloads/index.html"

echo "OK: Madde W3 — $TAG · $ASSET · run=$RUN"
