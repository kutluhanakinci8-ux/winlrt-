#!/usr/bin/env bash
# Madde H (strict): GitHub secret + imzalı Zip/Arc Windows CI success.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

test -f "$ROOT/scripts/sign-windows-nsis-bundle.ps1"
grep -q WINDOWS_CERTIFICATE "$ROOT/.github/workflows/lertiva-zip-windows.yml"
grep -q WINDOWS_CERTIFICATE "$ROOT/.github/workflows/lertiva-arc-windows.yml"

for wf in lertiva-zip-windows.yml lertiva-arc-windows.yml; do
  RUN="$(gh run list --workflow="$wf" --repo "$REPO" --branch main --limit 10 --json databaseId,conclusion \
    | jq -r '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty')"
  test -n "$RUN" || { echo "Son success yok: $wf" >&2; exit 1; }
  log="$(gh run view "$RUN" --repo "$REPO" --log 2>/dev/null || true)"
  if ! printf '%s\n' "$log" | grep -qE "Successfully signed|signtool sign"; then
    echo "Imza logu yok: $wf run=$RUN" >&2
    exit 1
  fi
done

bash "$ROOT/scripts/evaluate-lertiva-windows-signing-gate.sh"
jq -e '.gates.ci_zip_signed_build.pass and .gates.ci_arc_signed_build.pass' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json" >/dev/null

echo "OK: Madde H (strict) — imzalı Windows CI Zip + Arc"
