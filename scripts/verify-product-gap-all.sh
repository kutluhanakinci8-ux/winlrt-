#!/usr/bin/env bash
# Ürün gap özeti — repo kapılarından sonra insan/flag adımları.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-faz-all.sh"

bash "$ROOT/scripts/evaluate-lertiva-ev-ov-signing-gate.sh"
bash "$ROOT/scripts/evaluate-lertiva-macos-notarize-gate.sh"
bash "$ROOT/scripts/evaluate-lertiva-store-listing-gate.sh"
bash "$ROOT/scripts/evaluate-lertiva-mobile-mvp-gate.sh"

product_complete=0
product_total=4

jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json" >/dev/null && product_complete=$((product_complete + 1)) || echo "::warning::G2 ürün: EV/OV flag yok"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json" >/dev/null && product_complete=$((product_complete + 1)) || echo "::warning::G3 ürün: APPLE notarize flag yok"
jq -e '.gates.listing_url_set.pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-listing-gate.json" >/dev/null && product_complete=$((product_complete + 1)) || echo "::warning::G7: mağaza listing URL yok"

echo "OK: Repo G1–G10 · ürün kapıları $product_complete/$product_total (G8 LERTA prod ayrı — bkz. verify-faz-g8)"
