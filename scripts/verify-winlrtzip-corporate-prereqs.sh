#!/usr/bin/env bash
# W18: winlrtZIP kurumsal paket dosyalari.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  deploy/gpo/winlrtZIP.admx \
  deploy/gpo/en-US/winlrtZIP.adml \
  deploy/gpo/tr-TR/winlrtZIP.adml \
  deploy/gpo/README-winlrtZIP.md \
  scripts/winlrtzip-corporate-policy-sample.reg \
  scripts/windows/winlrtzip-corporate-pilot-preflight.ps1 \
  scripts/package-winlrtzip-corporate-gpo-zip.sh \
  docs/qa/WINLRTZIP_CORPORATE_PILOT_KOPYA.md \
  docs/qa/MADDE_W18_WINLRTZIP_CORPORATE.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q winlrtZIP "$ROOT/deploy/gpo/winlrtZIP.admx"
grep -q REG_KEY_PATH_ZIP "$ROOT/crates/winlrt-archive/src/enterprise.rs"
grep -q event_create "$ROOT/apps/winlrt-zip/src-tauri/src/lib.rs"

ZIP="$ROOT/apps/web/public/downloads/winlrtZIP-corporate-gpo.zip"
if [[ ! -f "$ZIP" ]]; then
  bash "$ROOT/scripts/package-winlrtzip-corporate-gpo-zip.sh"
fi
test -f "$ZIP"

echo "OK: winlrtZIP kurumsal paket (W18)"
