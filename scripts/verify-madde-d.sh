#!/usr/bin/env bash
set -euo pipefail
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

RUN="$(gh run list --workflow=winlrt-rar-windows.yml --repo "$REPO" --branch main --limit 20 --json databaseId,status,conclusion,event \
  | jq -r '[.[] | select(.event=="push" and .status=="completed" and .conclusion=="success")] | .[0].databaseId // empty')"
if [ -z "$RUN" ]; then
  echo "Madde D: başarılı beta workflow push koşusu yok" >&2
  exit 1
fi

REL="$(gh release list --repo "$REPO" --limit 30 --json tagName --jq '[.[].tagName | select(startswith("winlrt-rar-beta."))] | .[0] // empty')"
if [ -z "$REL" ]; then
  echo "Madde D: winlrt-rar-beta.* release yok" >&2
  exit 1
fi

echo "OK: Madde D — workflow run=$RUN release=$REL"
