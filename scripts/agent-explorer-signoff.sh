#!/usr/bin/env bash
# Pilot chat verisi → EXPLORER_QA_SIGNOFF.md (PASS). Agent çalıştırır.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md"

: "${EXPLORER_QA_DATE:=$(date +%Y-%m-%d)}"
: "${EXPLORER_QA_OS:?EXPLORER_QA_OS gerekli}"
: "${EXPLORER_QA_SHA256:?EXPLORER_QA_SHA256 gerekli}"
: "${EXPLORER_QA_BY:?EXPLORER_QA_BY gerekli}"
: "${EXPLORER_QA_TAG:=winlrt-rar-beta.42}"
NOTES="${EXPLORER_QA_NOTES:-}"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

cat >"$FILE" <<EOF
# Explorer pilot QA — imza kaydı (Faz 4)

Checklist: [WINLRTRAR_EXPLORER_QA.md](../WINLRTRAR_EXPLORER_QA.md)

| Alan | Değer |
|------|--------|
| Tarih | ${EXPLORER_QA_DATE} |
| Beta sürüm / Release | [${EXPLORER_QA_TAG}](https://github.com/${REPO}/releases/tag/${EXPLORER_QA_TAG}) |
| Pilot makine OS | ${EXPLORER_QA_OS} |
| Kurulum dosyası SHA256 | ${EXPLORER_QA_SHA256} |
| QA yapan | ${EXPLORER_QA_BY} |

Agent kaydı — chat **QA bitti**. Sonraki: [DALGA3_PILOT_OTURUM.md](./DALGA3_PILOT_OTURUM.md).

**Durum:** [x] PASS · [ ] FAIL${NOTES:+ (notlar: ${NOTES})}
EOF

echo "OK: $FILE güncellendi (PASS)"
