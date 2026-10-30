#!/usr/bin/env bash
# A→L + Post-v1 prep + Post-v2 (imza strict hariç).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

for v in \
  verify-madde-{c,d,ef,h,i,j,k,l,m,q,s-nsis,t-vps-doc,w0,w1,w2,w3,w4,w5,w6,w7,w8,w9,w10,w11,w12,w13,w14,w15,w16,w17,w18,w19,w20,w21,w22,w23,w24}.sh; do
  if [ -x "$ROOT/scripts/$v" ]; then
    bash "$ROOT/scripts/$v"
  fi
done

echo "OK: tüm verify-madde* (M strict ve imza sertifikası hariç)"
