#!/usr/bin/env bash
set -euo pipefail
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

pick_push_success() {
  local wf="$1"
  gh run list --workflow="$wf" --repo "$REPO" --branch main --limit 20 --json databaseId,status,conclusion,event \
    | jq -r '[.[] | select(.event=="push" and .status=="completed" and .conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true
}

RUN="$(pick_push_success lertiva-arc-windows.yml)"
if [ -z "$RUN" ]; then
  RUN="$(pick_push_success winlrt-rar-windows.yml)"
fi
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
