#!/usr/bin/env bash
# winlrtZIP W12 — Windows Explorer I1/I2 saha QA → WINLRTZIP_EXPLORER_QA_SIGNOFF.md (PASS).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/WINLRTZIP_EXPLORER_QA_SIGNOFF.md"

: "${WINLRTZIP_EXPLORER_QA_DATE:=$(date +%Y-%m-%d)}"
: "${WINLRTZIP_EXPLORER_QA_OS:?WINLRTZIP_EXPLORER_QA_OS gerekli}"
: "${WINLRTZIP_EXPLORER_QA_SHA256:?WINLRTZIP_EXPLORER_QA_SHA256 gerekli}"
: "${WINLRTZIP_EXPLORER_QA_BY:?WINLRTZIP_EXPLORER_QA_BY gerekli}"
: "${WINLRTZIP_EXPLORER_QA_TAG:=winlrt-zip-beta.1}"
NOTES="${WINLRTZIP_EXPLORER_QA_NOTES:-}"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

cat >"$FILE" <<EOF
# winlrtZIP Explorer I1/I2 — imza kaydı (W12)

Checklist: [WINLRTZIP_EXPLORER_QA_KOPYA.md](./WINLRTZIP_EXPLORER_QA_KOPYA.md) · Proxy otomasyon: [MADDE_W12](./MADDE_W12_WINLRTZIP_EXPLORER_I1_I2.md)

| Alan | Değer |
|------|--------|
| Tarih | ${WINLRTZIP_EXPLORER_QA_DATE} |
| Beta sürüm / Release | [${WINLRTZIP_EXPLORER_QA_TAG}](https://github.com/${REPO}/releases/tag/${WINLRTZIP_EXPLORER_QA_TAG}) |
| Pilot makine OS | ${WINLRTZIP_EXPLORER_QA_OS} |
| Kurulum dosyası SHA256 | ${WINLRTZIP_EXPLORER_QA_SHA256} |
| QA yapan | ${WINLRTZIP_EXPLORER_QA_BY} |

**Durum:** [x] PASS · [ ] FAIL${NOTES:+ (notlar: ${NOTES})}
EOF

echo "OK: $FILE güncellendi (PASS)"
