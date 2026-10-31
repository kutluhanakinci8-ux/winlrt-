#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
test -f "$ROOT/docs/qa/MADDE_G7_STORE_LISTING.md"
bash "$ROOT/scripts/verify-madde-w27.sh"
bash "$ROOT/scripts/verify-madde-w32.sh"
bash "$ROOT/scripts/evaluate-lertiva-store-listing-gate.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-listing-gate.json" >/dev/null
if ! jq -e '.gates.listing_url_set.pass == true' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-store-listing-gate.json" >/dev/null; then
  echo "::warning::G7: mağaza listing URL yok — lertiva-store-listing-url.txt"
fi
echo "OK: Faz G7 — mağaza repo kapıları (listing URL ayrı)"
