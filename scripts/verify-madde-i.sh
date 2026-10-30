#!/usr/bin/env bash
set -euo pipefail
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

TAG="$(gh release list --repo "$REPO" --limit 30 --json tagName,isPrerelease \
  | jq -r '[.[] | select(.isPrerelease and (.tagName | startswith("winlrt-rar-beta.")))] | .[0].tagName // empty')"
test -n "$TAG"

ASSET="$(gh release view "$TAG" --repo "$REPO" --json assets \
  | jq -r '.assets[] | select(.name | endswith("-setup.exe")) | .name' | head -1)"
test -n "$ASSET"

DIGEST="$(gh release view "$TAG" --repo "$REPO" --json assets \
  | jq -r '.assets[] | select(.name == "'"$ASSET"'") | .digest // empty' | sed 's/^sha256://')"
test -n "$DIGEST"

pick_arc_run() {
  local wf="$1"
  local run
  run="$(gh run list --workflow="$wf" --repo "$REPO" --branch main --limit 20 --json databaseId,conclusion,displayTitle \
    | jq -r '[.[] | select(.conclusion=="success" and (.displayTitle | test("trigger-beta-release|beta release"; "i")))] | .[0].databaseId // empty' 2>/dev/null || true)"
  if [[ -z "$run" ]]; then
    run="$(gh run list --workflow="$wf" --repo "$REPO" --branch main --limit 5 --json databaseId,conclusion \
      | jq -r '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty' 2>/dev/null || true)"
  fi
  printf '%s' "$run"
}
RUN="$(pick_arc_run lertiva-arc-windows.yml)"
if [[ -z "$RUN" ]]; then
  RUN="$(pick_arc_run winlrt-rar-windows.yml)"
fi
test -n "$RUN"

echo "OK: Madde I — $TAG · $ASSET · sha256=$DIGEST · run=$RUN"
