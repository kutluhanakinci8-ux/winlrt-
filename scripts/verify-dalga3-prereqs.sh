#!/usr/bin/env bash
# Dalga 3: kurumsal dosyalar + rehber (Linux CI / agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  deploy/gpo/Lertiva Arc.admx \
  scripts/winlrt-corporate-policy-sample.reg \
  scripts/windows/corporate-pilot-preflight.ps1 \
  scripts/windows/apply-corporate-sample.ps1 \
  deploy/gpo/README.md \
  scripts/package-corporate-gpo-zip.sh \
  scripts/windows/pilot-dalga3-start.ps1 \
  scripts/windows/pilot-dalga4-start.ps1 \
  scripts/windows/README.md \
  scripts/check-v1-gate-ready.sh \
  scripts/agent-v1-release-draft.sh \
  docs/qa/DALGA3_KURUMSAL_KOPYA.md \
  docs/qa/KURUMSAL_PILOT_SIGNOFF.md \
  docs/qa/FAZ5_KURUMSAL_BASLANGIC.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q 'PasswordPolicy' "$ROOT/deploy/gpo/Lertiva Arc.admx"
grep -q 'corporate' "$ROOT/scripts/winlrt-corporate-policy-sample.reg"

ZIP="$ROOT/apps/web/public/downloads/Lertiva Arc-corporate-gpo.zip"
if [ ! -f "$ZIP" ]; then
  bash "$ROOT/scripts/package-corporate-gpo-zip.sh"
fi
test -f "$ZIP"

echo "OK: Dalga 3 kurumsal paket (ADMX + ZIP + QA docs)"
