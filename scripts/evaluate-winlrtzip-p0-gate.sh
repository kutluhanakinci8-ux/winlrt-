#!/usr/bin/env bash
# Lertiva Zip P0×3 kapısı: sıkıştırma (Balanced zip_kpi) + hız (Fast zip_kpi) + güven (I4 AES).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BALANCED="${1:?bench JSON (balanced, --zip-kpi)}"
FAST="${2:?bench JSON (fast, --zip-kpi)}"
OUT="${3:-$ROOT/apps/web/public/benchmark/samples/winlrtzip-p0-gate.json}"
STRICT="${WINLRT_ZIP_KPI_STRICT:-${WINLRT_KPI_STRICT:-0}}"

command -v jq >/dev/null || { echo "jq gerekli" >&2; exit 1; }

extract_suite() {
  local f="$1"
  if jq -e '.zip_kpi' "$f" >/dev/null 2>&1; then
    cat "$f"
    return
  fi
  local raw start
  raw="$(tr -d '\r' <"$f")"
  start="${raw%%\{*}"
  raw="${raw#"$start"}"
  raw="${raw%"${raw##*\}}"}"
  printf '%s' "$raw"
}

for f in "$BALANCED" "$FAST"; do
  if [[ ! -f "$f" ]]; then
    echo "Dosya yok: $f" >&2
    exit 1
  fi
  if ! extract_suite "$f" | jq -e '.zip_kpi' >/dev/null; then
    echo "zip_kpi yok: $f (bench --zip-kpi kullanın)" >&2
    exit 1
  fi
done

B_JSON="$(extract_suite "$BALANCED")"
F_JSON="$(extract_suite "$FAST")"

comp_gate="$(echo "$B_JSON" | jq -r '.zip_kpi.compression_gate')"
speed_gate="$(echo "$F_JSON" | jq -r '.zip_kpi.speed_gate // empty')"
b_platform="$(echo "$B_JSON" | jq -r '.platform')"
f_platform="$(echo "$F_JSON" | jq -r '.platform')"
generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"

security_gate=true
if ! grep -q 'AES-256' "$ROOT/crates/lertiva-archive/src/zip_io.rs"; then
  security_gate=false
fi
if ! (cd "$ROOT" && cargo test -p lertiva-archive --test winlrtzip_interop i4_aes256_roundtrip --quiet); then
  security_gate=false
fi

all_pass=true
[[ "$comp_gate" == "true" ]] || all_pass=false
[[ "$security_gate" == "true" ]] || all_pass=false
if [[ -n "$speed_gate" ]]; then
  [[ "$speed_gate" == "true" ]] || all_pass=false
else
  all_pass=false
fi

mkdir -p "$(dirname "$OUT")"
jq -n \
  --arg product "Lertiva Zip" \
  --arg generated_at "$generated_at" \
  --argjson compression_gate "$([[ "$comp_gate" == "true" ]] && echo true || echo false)" \
  --argjson speed_gate "$([[ "$speed_gate" == "true" ]] && echo true || echo false)" \
  --argjson security_gate "$([[ "$security_gate" == "true" ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == "true" ]] && echo true || echo false)" \
  --arg balanced_platform "$b_platform" \
  --arg fast_platform "$f_platform" \
  --arg balanced_path "$BALANCED" \
  --arg fast_path "$FAST" \
  '{
    product: $product,
    generated_at: $generated_at,
    disclaimer: "P0×3: sıkıştırma (Balanced zip_kpi), hız (Fast zip_kpi), güven (AES I4). Resmi Windows: import-windows-zip-kpi-to-vitrin.sh",
    p0: {
      compression: { gate: $compression_gate, profile: "Balanced", platform: $balanced_platform },
      speed: { gate: $speed_gate, profile: "Fast", platform: $fast_platform },
      security: { gate: $security_gate, checks: ["zip_io AES-256", "interop i4_aes256_roundtrip"] }
    },
    all_pass: $all_pass,
    sources: { balanced_json: $balanced_path, fast_json: $fast_path }
  }' >"$OUT"

echo "Lertiva Zip P0×3 — sıkıştırma: $comp_gate · hız: ${speed_gate:-ölçülmedi} · güven: $security_gate · tümü: $all_pass"
echo "Yazıldı: $OUT"

if [[ "$all_pass" != "true" ]]; then
  echo "::warning::Lertiva Zip P0×3 kapısı henüz tam geçmedi."
  if [[ "$STRICT" == "1" ]]; then
    exit 1
  fi
fi
exit 0
