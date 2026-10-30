#!/usr/bin/env bash
# Chat "v1.0 hazir" → taslak release notu (tag agent/siz onayı ile).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/check-v1-gate-ready.sh"

: "${V1_TAG:=v1.0.0}"
NOTES="${V1_NOTES:-Lertiva Arc v1.0 — Explorer + kurumsal pilot onaylı.}"
OUT="$ROOT/docs/qa/V1_RELEASE_DRAFT.md"

cat >"$OUT" <<EOF
# v1.0 release taslağı (agent)

**Tag:** \`${V1_TAG}\`

## Notlar

${NOTES}

## Kapı kontrol

- Explorer: PASS ([EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md))
- Kurumsal: PASS ([KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md))
- İmza/KPI: [DALGA4_V1_OTURUM.md](./DALGA4_V1_OTURUM.md)

Tag oluşturma: onay sonrası \`git tag ${V1_TAG}\` + GitHub Release + vitrin duyurusu.
EOF

echo "OK: $OUT"
