#!/usr/bin/env bash
# G2 ürün — EV/OV imza (flag dosyası ile tamamlanır).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json}"
FLAG="$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag"

dev_ok=false
bash "$ROOT/scripts/verify-madde-h-signed.sh" >/dev/null 2>&1 && dev_ok=true

ev_ov=false
[[ -f "$FLAG" ]] && grep -q 'ev_ov' "$FLAG" && ev_ov=true

all_pass=false
[[ "$dev_ok" == true && "$ev_ov" == true ]] && all_pass=true

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson dev "$([[ "$dev_ok" == true ]] && echo true || echo false)" \
  --argjson ev_ov "$([[ "$ev_ov" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Windows",
    generated_at: $generated_at,
    disclaimer: "EV/OV: vitrin-ev-ov-configured.flag ile isaretlenir.",
    gates: {
      dev_authenticode_ci: { pass: $dev },
      ev_ov_pfx_production: { pass: $ev_ov, flag: "lertiva-ev-ov-configured.flag" }
    },
    all_pass: $all_pass
  }' >"$OUT"
echo "G2 EV/OV — dev CI: $dev_ok · EV/OV flag: $ev_ov · tümü: $all_pass"
echo "Yazıldı: $OUT"
