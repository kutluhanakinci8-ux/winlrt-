#!/usr/bin/env bash
# W13: KOBİ pilot dosyalari (Linux CI / agent).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

for f in \
  scripts/windows/pilot-lertiva-zip-kobi-start.ps1 \
  scripts/windows/lertiva-zip-kobi-pilot-preflight.ps1 \
  scripts/agent-lertiva-zip-kobi-signoff.sh \
  docs/qa/WINLRTZIP_KOBI_PILOT_KOPYA.md \
  docs/qa/WINLRTZIP_KOBI_PILOT_SIGNOFF.md \
  docs/qa/MADDE_W13_WINLRTZIP_KOBI_PILOT.md
do
  test -f "$ROOT/$f" || { echo "Eksik: $f" >&2; exit 1; }
done

grep -q 'EmailAttachment\|e-posta\|email' "$ROOT/docs/qa/WINLRTZIP_KOBI_PILOT_KOPYA.md"
grep -q kobi_invoice_bundle "$ROOT/crates/lertiva-archive/tests/lertiva_zip_interop.rs"

echo "OK: Lertiva Zip KOBİ pilot paketi (W13)"
