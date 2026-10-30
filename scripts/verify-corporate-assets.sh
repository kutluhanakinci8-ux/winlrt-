#!/usr/bin/env bash
# Dalga 3 öncesi: kurumsal dosyalar repo'da (Linux CI smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -f "$ROOT/deploy/gpo/winlrtRAR.admx"
test -f "$ROOT/scripts/winlrt-corporate-policy-sample.reg"
echo "OK: ADMX + örnek .reg mevcut"
