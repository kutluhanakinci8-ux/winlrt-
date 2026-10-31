#!/usr/bin/env bash
# ev_ov flag yalnizca kurumsal EV/OV sonrasi; dev self-signed ile isaretlenmesin.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag"
DEV_CER="$ROOT/.github/signing/lertiva-dev-codesign.cer"

[[ -f "$FLAG" ]] || exit 0
grep -q 'ev_ov' "$FLAG" || exit 0

if [[ ! -f "$DEV_CER" ]]; then
  echo "OK: ev_ov flag var, dev .cer yok — varsayilan EV/OV"
  exit 0
fi

subject="$(openssl x509 -in "$DEV_CER" -noout -subject 2>/dev/null \
  || openssl x509 -inform DER -in "$DEV_CER" -noout -subject 2>/dev/null || true)"
if echo "$subject" | grep -qi 'Dev Code Signing'; then
  echo "FAIL: lertiva-ev-ov-configured.flag set ama repodaki sertifika hâlâ dev self-signed." >&2
  echo "      EV/OV PFX secret sonrasi flag isaretleyin veya flag silin." >&2
  exit 1
fi
echo "OK: ev_ov flag — dev sertifika eslesmedi (EV/OV kabul)"
