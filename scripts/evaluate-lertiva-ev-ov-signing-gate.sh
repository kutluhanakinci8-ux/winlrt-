#!/usr/bin/env bash
# G2 ürün — EV/OV imza (flag dosyası ile tamamlanır).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json}"
FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag"

dev_ok=false
bash "$ROOT/scripts/verify-madde-h-signed.sh" >/dev/null 2>&1 && dev_ok=true

ev_ov=false
if [[ -f "$FLAG" ]] && grep -q 'ev_ov' "$FLAG"; then
  if bash "$ROOT/scripts/verify-ev-ov-flag-honesty.sh" >/dev/null 2>&1; then
    ev_ov=true
  else
    echo "::warning::ev_ov flag var ama dev sertifika — EV/OV sayilmadi" >&2
  fi
fi

cert_tier="dev"
subject=""
if [[ -f "$ROOT/.github/signing/lertiva-dev-codesign.cer" ]]; then
  subject="$(openssl x509 -in "$ROOT/.github/signing/lertiva-dev-codesign.cer" -noout -subject 2>/dev/null \
    || openssl x509 -inform DER -in "$ROOT/.github/signing/lertiva-dev-codesign.cer" -noout -subject 2>/dev/null || true)"
  if echo "$subject" | grep -qi 'Dev Code Signing'; then
    cert_tier="dev_self_signed"
  else
    cert_tier="org_signing_cert"
  fi
fi

all_pass=false
[[ "$dev_ok" == true && "$ev_ov" == true ]] && all_pass=true

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson dev "$([[ "$dev_ok" == true ]] && echo true || echo false)" \
  --argjson ev_ov "$([[ "$ev_ov" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  --arg cert_tier "$cert_tier" \
  --arg subject "$subject" \
  '{
    product: "Lertiva Windows",
    generated_at: $generated_at,
    disclaimer: "EV/OV: kurumsal PFX + flag; dev TAURI/WINDOWS secret EV sayilmaz.",
    certificate_tier: $cert_tier,
    certificate_subject: (if $subject == "" then null else $subject end),
    gates: {
      dev_authenticode_ci: { pass: $dev },
      ev_ov_pfx_production: { pass: $ev_ov, flag: "lertiva-ev-ov-configured.flag" }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G2 EV/OV — dev CI: $dev_ok · EV/OV flag: $ev_ov · tümü: $all_pass"
echo "Yazıldı: $OUT"
