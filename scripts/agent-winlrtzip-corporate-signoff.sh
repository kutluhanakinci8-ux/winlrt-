#!/usr/bin/env bash
# Lertiva Zip W19 — kurumsal saha pilot → WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md (PASS).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md"

: "${WINLRTZIP_CORP_SEGMENT:?WINLRTZIP_CORP_SEGMENT gerekli (banka/sendika/KOBİ)}"
: "${WINLRTZIP_CORP_DATE:=$(date +%Y-%m-%d)}"
: "${WINLRTZIP_CORP_BY:?WINLRTZIP_CORP_BY gerekli}"
: "${WINLRTZIP_CORP_POLICY:?WINLRTZIP_CORP_POLICY gerekli (ADMX veya .reg)}"
AUDIT="${WINLRTZIP_CORP_AUDIT:-cargo test audit:: — parola alanı yok}"
NOTES="${WINLRTZIP_CORP_NOTES:-}"

cat >"$FILE" <<EOF
# Lertiva Zip kurumsal pilot — imza kaydı (W19)

Rehber: [WINLRTZIP_CORPORATE_PILOT_KOPYA.md](./WINLRTZIP_CORPORATE_PILOT_KOPYA.md) · GPO: [MADDE_W18](./MADDE_W18_WINLRTZIP_CORPORATE.md)

| Alan | Değer |
|------|--------|
| Segment | ${WINLRTZIP_CORP_SEGMENT} |
| Tarih | ${WINLRTZIP_CORP_DATE} |
| ADMX / .reg | ${WINLRTZIP_CORP_POLICY} |
| Denetim testi | ${AUDIT} |
| Onaylayan | ${WINLRTZIP_CORP_BY} |

**Durum:** [x] PASS · [ ] FAIL${NOTES:+ (notlar: ${NOTES})}
EOF

echo "OK: $FILE güncellendi (PASS)"
