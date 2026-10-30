#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

grep -q 'windows-official-ci' "$ROOT/apps/web/public/benchmark/samples/windows-balanced-kpi.json"
grep -q '"compression_gate": true' "$ROOT/apps/web/public/benchmark/samples/windows-balanced-kpi.json"

RUN="$(gh run list --workflow=bench-windows.yml --repo "$REPO" --branch main --limit 15 --json databaseId,status,conclusion,event \
  | jq -r '[.[] | select(.event=="push" and .status=="completed" and .conclusion=="success")] | .[0].databaseId // empty')"
if [ -z "$RUN" ]; then
  echo "Madde C: başarılı Windows bench push koşusu yok" >&2
  exit 1
fi

echo "OK: Madde C — vitrin KPI + Windows bench success run=$RUN"
