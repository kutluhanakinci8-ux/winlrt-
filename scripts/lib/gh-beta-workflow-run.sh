#!/usr/bin/env bash
# GitHub Actions: yeniden adlandırılmış workflow için son başarılı main koşusu (geçiş dönemi fallback).
# Usage: REPO=... source scripts/lib/gh-beta-workflow-run.sh
#        gh_beta_workflow_run lertiva-zip-windows.yml winlrt-zip-windows.yml
gh_beta_workflow_run() {
  local wf="$1" legacy="${2:-}" repo="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
  local run
  run="$(gh run list --workflow="$wf" --repo "$repo" --branch main --limit 15 --json databaseId,conclusion,event \
    | jq -r '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true)"
  if [[ -z "$run" && -n "$legacy" ]]; then
    run="$(gh run list --workflow="$legacy" --repo "$repo" --branch main --limit 15 --json databaseId,conclusion,event \
      | jq -r '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true)"
  fi
  printf '%s' "$run"
}
