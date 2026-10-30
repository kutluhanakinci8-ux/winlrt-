#!/usr/bin/env bash
# Dalga 3: kurumsal dosyalar + rehber (Linux CI / agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  deploy/gpo/winlrtRAR.admx \
  scripts/winlrt-corporate-policy-sample.reg \
  scripts/windows/corporate-pilot-preflight.ps1 \
  docs/qa/DALGA3_KURUMSAL_KOPYA.md \
  docs/qa/KURUMSAL_PILOT_SIGNOFF.md \
  docs/qa/FAZ5_KURUMSAL_BASLANGIC.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q 'PasswordPolicy' "$ROOT/deploy/gpo/winlrtRAR.admx"
grep -q 'corporate' "$ROOT/scripts/winlrt-corporate-policy-sample.reg"

echo "OK: Dalga 3 kurumsal paket (ADMX + .reg + QA docs)"
