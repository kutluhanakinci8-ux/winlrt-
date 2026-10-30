#!/usr/bin/env bash
# Pilot chat verisi → EXPLORER_QA_SIGNOFF.md (PASS). Agent çalıştırır.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md"

: "${EXPLORER_QA_DATE:=$(date +%Y-%m-%d)}"
: "${EXPLORER_QA_OS:?EXPLORER_QA_OS gerekli}"
: "${EXPLORER_QA_SHA256:?EXPLORER_QA_SHA256 gerekli}"
: "${EXPLORER_QA_BY:?EXPLORER_QA_BY gerekli}"
NOTES="${EXPLORER_QA_NOTES:-}"

cat >"$FILE" <<EOF
# Explorer pilot QA — imza kaydı (Faz 4)

Checklist: [WINLRTRAR_EXPLORER_QA.md](../WINLRTRAR_EXPLORER_QA.md)

| Alan | Değer |
|------|--------|
| Tarih | ${EXPLORER_QA_DATE} |
| Beta sürüm / Release | [winlrt-rar-beta.41064653814](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814) |
| Pilot makine OS | ${EXPLORER_QA_OS} |
| Kurulum dosyası SHA256 | ${EXPLORER_QA_SHA256} |
| QA yapan | ${EXPLORER_QA_BY} |

Agent kaydı — chat **QA bitti**. Sonraki: [DALGA3_PILOT_OTURUM.md](./DALGA3_PILOT_OTURUM.md).

**Durum:** [x] PASS · [ ] FAIL${NOTES:+ (notlar: ${NOTES})}
EOF

echo "OK: $FILE güncellendi (PASS)"
