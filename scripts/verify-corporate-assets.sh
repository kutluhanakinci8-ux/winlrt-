#!/usr/bin/env bash
# Dalga 3 öncesi: kurumsal dosyalar repo'da (Linux CI smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -f "$ROOT/deploy/gpo/Lertiva Arc.admx"
test -f "$ROOT/deploy/gpo/en-US/Lertiva Arc.adml"
test -f "$ROOT/deploy/gpo/tr-TR/Lertiva Arc.adml"
test -f "$ROOT/scripts/winlrt-corporate-policy-sample.reg"
echo "OK: ADMX + ADML (en/tr) + örnek .reg mevcut"
