#!/usr/bin/env bash
# W18: Lertiva Zip kurumsal paket dosyalari.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  deploy/gpo/Lertiva Zip.admx \
  deploy/gpo/en-US/Lertiva Zip.adml \
  deploy/gpo/tr-TR/Lertiva Zip.adml \
  deploy/gpo/README-Lertiva Zip.md \
  scripts/winlrtzip-corporate-policy-sample.reg \
  scripts/windows/winlrtzip-corporate-pilot-preflight.ps1 \
  scripts/package-winlrtzip-corporate-gpo-zip.sh \
  docs/qa/WINLRTZIP_CORPORATE_PILOT_KOPYA.md \
  docs/qa/MADDE_W18_WINLRTZIP_CORPORATE.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q Lertiva Zip "$ROOT/deploy/gpo/Lertiva Zip.admx"
grep -q REG_KEY_PATH_ZIP "$ROOT/crates/lertiva-archive/src/enterprise.rs"
grep -q event_create "$ROOT/apps/lertiva-zip/src-tauri/src/lib.rs"

ZIP="$ROOT/apps/web/public/downloads/Lertiva Zip-corporate-gpo.zip"
if [[ ! -f "$ZIP" ]]; then
  bash "$ROOT/scripts/package-winlrtzip-corporate-gpo-zip.sh"
fi
test -f "$ZIP"

echo "OK: Lertiva Zip kurumsal paket (W18)"
