#!/usr/bin/env bash
# Lertiva geliştirme kod imzası (self-signed) — gerçek EV/OV gelene kadar.
# Private key repoya GİRMEZ; yalnızca public .cer commit edilir.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="${LERTIVA_SIGNING_DIR:-$ROOT/.signing-local}"
PUBLIC_DIR="$ROOT/.github/signing"
DAYS="${LERTIVA_DEV_CODESIGN_DAYS:-825}"
CN="${LERTIVA_DEV_CODESIGN_CN:-Lertiva Dev Code Signing}"
PASS="${LERTIVA_DEV_CODESIGN_PASSWORD:-}"

if [[ -z "$PASS" ]]; then
  echo "Parola gerekli: LERTIVA_DEV_CODESIGN_PASSWORD='…' $0" >&2
  exit 1
fi

mkdir -p "$OUT_DIR" "$PUBLIC_DIR"
CFG="$OUT_DIR/openssl-codesign.cnf"
cat >"$CFG" <<EOF
[req]
distinguished_name=req_dn
x509_extensions=v3_codesign
prompt=no

[req_dn]
CN=$CN
O=Lertiva
C=TR

[v3_codesign]
basicConstraints=critical,CA:FALSE
keyUsage=critical,digitalSignature
extendedKeyUsage=critical,codeSigning
subjectKeyIdentifier=hash
EOF

KEY="$OUT_DIR/lertiva-dev-codesign.key.pem"
CRT="$OUT_DIR/lertiva-dev-codesign.crt.pem"
PFX="$OUT_DIR/lertiva-dev-codesign.pfx"
CER="$PUBLIC_DIR/lertiva-dev-codesign.cer"

openssl req -x509 -newkey rsa:3072 -sha256 -days "$DAYS" -nodes \
  -keyout "$KEY" -out "$CRT" -config "$CFG" -extensions v3_codesign 2>/dev/null

openssl pkcs12 -export -out "$PFX" -inkey "$KEY" -in "$CRT" \
  -passout "pass:${PASS}" -name "$CN"

openssl x509 -in "$CRT" -outform DER -out "$CER"

chmod 600 "$KEY" "$PFX"
echo "OK: private PFX → $PFX (gitignore)"
echo "OK: public CER → $CER (commit edilebilir)"
echo ""
echo "GitHub Actions:"
echo "  base64 -w0 < \"$PFX\"   # veya: bash scripts/push-lertiva-dev-signing-secrets.sh"
echo "Windows güven (isteğe bağlı, yönetici PS):"
echo "  Import-Certificate -FilePath \"$CER\" -CertStoreLocation Cert:\\LocalMachine\\Root"
echo "  Import-Certificate -FilePath \"$CER\" -CertStoreLocation Cert:\\LocalMachine\\TrustedPublisher"
