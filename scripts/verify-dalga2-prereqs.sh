#!/usr/bin/env bash
# Dalga 2: repo + vitrin kaynağı beta/QA hazır (Linux CI / agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARC_TAG="v1.0.1"
# Vitrin: manifest tabanlı indirme (varsayılan v1.0.1 Lertiva Arc installer).

for f in \
  docs/qa/DALGA2_EXPLORER_QA_KOPYA.md \
  docs/qa/EXPLORER_QA_SIGNOFF.md \
  docs/WINLRTRAR_EXPLORER_QA.md \
  docs/qa/FAZ4_PILOT_BASLANGIC.md \
  scripts/windows/explorer-qa-preflight.ps1 \
  apps/web/public/pilot/index.html \
  scripts/generate-pilot-status-json.sh \
  scripts/agent-parse-pilot-chat.sh \
  scripts/windows/pilot-dalga2-start.ps1
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

if ! grep -qE 'lertiva-downloads-manifest|releases/tag/v1\.0\.(0|1)|Lertiva Arc' "$ROOT/apps/web/public/downloads/index.html"; then
  echo "Eksik: downloads/index.html Lertiva Arc / manifest referansı" >&2
  exit 1
fi
test -f "$ROOT/apps/web/public/downloads/lertiva-downloads-manifest.json"

if command -v curl >/dev/null 2>&1; then
  if ! curl -fsSL -o /dev/null -w '' \
    "https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/${ARC_TAG}" 2>/dev/null; then
    ARC_TAG="v1.0.0"
    curl -fsSL -o /dev/null -w '' \
      "https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/${ARC_TAG}"
  fi
fi

echo "OK: Dalga 2 önkoşullar (docs + vitrin link + release erişilebilir)"
