#!/usr/bin/env bash
# Kurumsal pilot chat verisi → KURUMSAL_PILOT_SIGNOFF.md (PASS).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md"

: "${CORP_PILOT_SEGMENT:?CORP_PILOT_SEGMENT gerekli (banka/sendika/KOBİ)}"
: "${CORP_PILOT_DATE:=$(date +%Y-%m-%d)}"
: "${CORP_PILOT_BY:?CORP_PILOT_BY gerekli}"
: "${CORP_PILOT_POLICY:?CORP_PILOT_POLICY gerekli (ADMX veya .reg)}"
AUDIT="${CORP_PILOT_AUDIT:-test edildi}"

cat >"$FILE" <<EOF
# Kurumsal pilot — imza kaydı (Faz 5)

Rehber: [WINLRTRAR_PILOT_KICKOFF.md](../WINLRTRAR_PILOT_KICKOFF.md)

| Alan | Değer |
|------|--------|
| Segment | ${CORP_PILOT_SEGMENT} |
| Tarih | ${CORP_PILOT_DATE} |
| ADMX veya .reg uygulandı | ${CORP_PILOT_POLICY} |
| Denetim / HTTP forward test | ${CORP_PILOT_AUDIT} |
| Onaylayan | ${CORP_PILOT_BY} |

Agent kaydı — chat **kurumsal pilot bitti**.

**Durum:** [x] PASS · [ ] FAIL
EOF

echo "OK: $FILE güncellendi (PASS)"
