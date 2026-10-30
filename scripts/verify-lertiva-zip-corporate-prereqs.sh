#!/usr/bin/env bash
# W18: Lertiva Zip kurumsal paket dosyalari.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  deploy/gpo/lertiva-zip.admx \
  deploy/gpo/en-US/lertiva-zip.adml \
  deploy/gpo/tr-TR/lertiva-zip.adml \
  deploy/gpo/README-lertiva-zip.md \
  scripts/lertiva-zip-corporate-policy-sample.reg \
  scripts/windows/lertiva-zip-corporate-pilot-preflight.ps1 \
  scripts/package-lertiva-zip-corporate-gpo-zip.sh \
  docs/qa/WINLRTZIP_CORPORATE_PILOT_KOPYA.md \
  docs/qa/MADDE_W18_WINLRTZIP_CORPORATE.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q Lertiva Zip "$ROOT/deploy/gpo/lertiva-zip.admx"
grep -q REG_KEY_PATH_ZIP "$ROOT/crates/lertiva-archive/src/enterprise.rs"
grep -q event_create "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"

ZIP="$ROOT/apps/web/public/downloads/lertiva-zip-corporate-gpo.zip"
if [[ ! -f "$ZIP" ]]; then
  bash "$ROOT/scripts/package-lertiva-zip-corporate-gpo-zip.sh"
fi
test -f "$ZIP"

echo "OK: Lertiva Zip kurumsal paket (W18)"
