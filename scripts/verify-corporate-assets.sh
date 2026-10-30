#!/usr/bin/env bash
# Dalga 3 öncesi: kurumsal dosyalar repo'da (Linux CI smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -f "$ROOT/deploy/gpo/lertiva-arc.admx"
test -f "$ROOT/deploy/gpo/en-US/lertiva-arc.adml"
test -f "$ROOT/deploy/gpo/tr-TR/lertiva-arc.adml"
test -f "$ROOT/scripts/lertiva-arc-corporate-policy-sample.reg"
echo "OK: ADMX + ADML (en/tr) + örnek .reg mevcut"
