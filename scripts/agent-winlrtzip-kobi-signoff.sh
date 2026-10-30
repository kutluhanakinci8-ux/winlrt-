#!/usr/bin/env bash
# winlrtZIP W13 — KOBİ saha pilot → WINLRTZIP_KOBI_PILOT_SIGNOFF.md (PASS).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FILE="$ROOT/docs/qa/WINLRTZIP_KOBI_PILOT_SIGNOFF.md"

: "${WINLRTZIP_KOBI_DATE:=$(date +%Y-%m-%d)}"
: "${WINLRTZIP_KOBI_OS:?WINLRTZIP_KOBI_OS gerekli}"
: "${WINLRTZIP_KOBI_SHA256:?WINLRTZIP_KOBI_SHA256 gerekli}"
: "${WINLRTZIP_KOBI_BY:?WINLRTZIP_KOBI_BY gerekli}"
: "${WINLRTZIP_KOBI_SEGMENT:=KOBİ}"
: "${WINLRTZIP_KOBI_TAG:=winlrt-zip-beta.1}"
NOTES="${WINLRTZIP_KOBI_NOTES:-}"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

cat >"$FILE" <<EOF
# winlrtZIP KOBİ pilot — imza kaydı (W13)

Checklist: [WINLRTZIP_KOBI_PILOT_KOPYA.md](./WINLRTZIP_KOBI_PILOT_KOPYA.md) · Proxy: [MADDE_W13](./MADDE_W13_WINLRTZIP_KOBI_PILOT.md)

| Alan | Değer |
|------|--------|
| Tarih | ${WINLRTZIP_KOBI_DATE} |
| Segment | ${WINLRTZIP_KOBI_SEGMENT} |
| Beta / Release | [${WINLRTZIP_KOBI_TAG}](https://github.com/${REPO}/releases/tag/${WINLRTZIP_KOBI_TAG}) |
| Pilot makine OS | ${WINLRTZIP_KOBI_OS} |
| Kurulum SHA256 | ${WINLRTZIP_KOBI_SHA256} |
| QA yapan | ${WINLRTZIP_KOBI_BY} |

**Durum:** [x] PASS · [ ] FAIL${NOTES:+ (notlar: ${NOTES})}
EOF

echo "OK: $FILE güncellendi (PASS)"
