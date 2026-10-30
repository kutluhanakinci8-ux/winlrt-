#!/usr/bin/env bash
# Madde B: son main pipeline success mi?
set -euo pipefail
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
RUN="$(gh run list --workflow=main-pipeline.yml --repo "$REPO" --branch main --limit 20 --json databaseId,status,conclusion,event \
  | jq -r '[.[] | select(.event=="push" and .status=="completed" and .conclusion=="success")] | .[0].databaseId // empty')"
if [ -z "$RUN" ]; then
  echo "Madde B: henüz success push koşusu yok (cancel veya in_progress)" >&2
  exit 1
fi
echo "OK: Madde B — main pipeline success run id=$RUN"
gh run view "$RUN" --repo "$REPO" --json jobs --jq '.jobs[] | "\(.name): \(.conclusion)"'
