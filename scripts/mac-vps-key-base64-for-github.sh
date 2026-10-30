#!/usr/bin/env bash
# Mac: Private key → tek satır Base64 (Secret adı: VPS_SSH_KEY_B64)
# Bazı tarayıcılarda çok satırlı yapıştırma bozulur; tek satır daha kolay olabilir.
set -euo pipefail

KEY="${HOME}/.ssh/winlrt-gha-deploy"
OUT="${HOME}/Desktop/VPS_SSH_KEY_B64.txt"

if [[ ! -f "$KEY" ]]; then
  echo "Anahtar yok: $KEY"
  exit 1
fi

base64 -i "$KEY" | tr -d '\n' >"$OUT"
echo "Dosya: $OUT"
echo "GitHub → Settings → Secrets → New secret"
echo "  Name:  VPS_SSH_KEY_B64"
echo "  Value: bu dosyadaki tek satır (Finder'da çift tık → Cmd+A → kopyala)"
open -R "$OUT" 2>/dev/null || true
