#!/usr/bin/env bash
# Dalga 2: repo + vitrin kaynağı beta/QA hazır (Linux CI / agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="winlrt-rar-beta.41064653814"

for f in \
  docs/qa/DALGA2_EXPLORER_QA_KOPYA.md \
  docs/qa/EXPLORER_QA_SIGNOFF.md \
  docs/WINLRTRAR_EXPLORER_QA.md \
  docs/qa/FAZ4_PILOT_BASLANGIC.md \
  scripts/windows/explorer-qa-preflight.ps1 \
  apps/web/public/pilot/index.html \
  scripts/generate-pilot-status-json.sh \
  scripts/agent-parse-pilot-chat.sh
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q "$TAG" "$ROOT/apps/web/public/downloads/index.html"

if command -v curl >/dev/null 2>&1; then
  curl -fsSL -o /dev/null -w '' \
    "https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/${TAG}"
fi

echo "OK: Dalga 2 önkoşullar (docs + vitrin link + release erişilebilir)"
