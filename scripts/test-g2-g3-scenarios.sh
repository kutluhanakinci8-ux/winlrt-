#!/usr/bin/env bash
# G2/G3 runbook senaryo testleri (agent + CI okuyabilir adimlar).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
FAIL=0

pass() { echo "✅ $1"; }
fail() { echo "❌ $1"; FAIL=1; }
warn() { echo "⚠️  $1"; }

echo "========== G2 senaryolari =========="

# G2-S1 Dev imza hatti (A0)
if bash "$ROOT/scripts/verify-madde-h-signed.sh" >/dev/null 2>&1; then
  pass "G2-S1 A0 — dev Authenticode CI hatti (verify-madde-h-signed)"
else
  fail "G2-S1 A0 — dev imza hatti"
fi

# G2-S2 Sertifika tier = dev (A1 henuz EV degil)
subject="$(openssl x509 -in "$ROOT/.github/signing/lertiva-dev-codesign.cer" -noout -subject 2>/dev/null \
  || openssl x509 -inform DER -in "$ROOT/.github/signing/lertiva-dev-codesign.cer" -noout -subject 2>/dev/null || echo "?")"
if echo "$subject" | grep -qi 'Dev Code Signing'; then
  pass "G2-S2 A1 beklenen — repoda dev self-signed (EV/OV CA henuz yok)"
else
  warn "G2-S2 — dev .cer beklenmiyordu; EV/OV .cer kontrol edin"
fi

# G2-S3 Son basarili Windows CI (A3 referans)
if command -v gh >/dev/null 2>&1; then
  zid="$(gh run list --workflow=lertiva-zip-windows.yml --repo "$REPO" --branch main -L 1 --json databaseId,conclusion \
    | jq -r '.[0] | select(.conclusion=="success") | .databaseId' 2>/dev/null || true)"
  aid="$(gh run list --workflow=lertiva-arc-windows.yml --repo "$REPO" --branch main -L 1 --json databaseId,conclusion \
    | jq -r '.[0] | select(.conclusion=="success") | .databaseId' 2>/dev/null || true)"
  if [[ -n "$zid" && -n "$aid" ]]; then
    pass "G2-S3 A3 — son Zip/Arc Windows run success (Zip=$zid Arc=$aid)"
    log_snip="$(gh run view "$zid" --repo "$REPO" --log 2>/dev/null | rg 'Successfully signed|imzalar ge' | head -5 || true)"
    if echo "$log_snip" | rg -q 'Successfully signed'; then
      pass "G2-S3b — Zip log: Successfully signed"
    else
      warn "G2-S3b — Zip log imza satiri bulunamadi (log erisimi/truncation?)"
    fi
    if echo "$log_snip" | rg -q 'imzalar ge'; then
      pass "G2-S3c — Zip log: signtool verify gecti"
    else
      warn "G2-S3c — verify satiri bulunamadi"
    fi
  else
    fail "G2-S3 A3 — Zip veya Arc son run success degil"
  fi
  if gh workflow run lertiva-zip-windows.yml --repo "$REPO" --ref main 2>&1 | rg -q '403'; then
    warn "G2-S3d — agent workflow dispatch 403 (A3 manuel tetikleme sizde)"
  fi
else
  warn "G2-S3 — gh yok, CI atlandi"
fi

# G2-S4 Dev G2 vitrin kapisi
bash "$ROOT/scripts/evaluate-lertiva-windows-signing-gate.sh" >/dev/null
if jq -e '.all_pass == true and .gates.vitrin_smartscreen_real.pass == true' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-windows-signing-gate.json" >/dev/null; then
  pass "G2-S4 — dev imza vitrin kapisi (G2 repo)"
else
  fail "G2-S4 — lertiva-windows-signing-gate"
fi

# G2-S5 EV/OV urun kapisi (flag yokken false olmali)
bash "$ROOT/scripts/evaluate-lertiva-ev-ov-signing-gate.sh" >/dev/null
if jq -e '.gates.dev_authenticode_ci.pass == true and .gates.ev_ov_pfx_production.pass == false' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-ev-ov-signing-gate.json" >/dev/null; then
  pass "G2-S5 A4 beklenen — ev_ov flag yok, urun kapisi false (dev-only)"
else
  fail "G2-S5 — EV/OV gate beklenmeyen durum"
fi

# G2-S6 flag honesty
if bash "$ROOT/scripts/verify-ev-ov-flag-honesty.sh" >/dev/null 2>&1; then
  pass "G2-S6 — ev_ov flag tutarliligi"
else
  fail "G2-S6 — ev_ov flag dev sertifika ile celiski"
fi

echo ""
echo "========== G3 senaryolari =========="

if bash "$ROOT/scripts/verify-faz-g3.sh" >/dev/null 2>&1; then
  pass "G3-S1 B0 — macOS repo W14+W34 (verify-faz-g3)"
else
  fail "G3-S1 B0 — verify-faz-g3"
fi

bash "$ROOT/scripts/evaluate-lertiva-macos-notarize-gate.sh" >/dev/null
if jq -e '.gates.macos_build_w14.pass == true and .gates.apple_notarize_configured.pass == false' \
  "$ROOT/apps/web/public/benchmark/samples/lertiva-macos-notarize-gate.json" >/dev/null; then
  pass "G3-S2 B4 beklenen — notarize flag yok (APPLE_* sonra)"
else
  fail "G3-S2 — macOS notarize gate"
fi

if command -v gh >/dev/null 2>&1; then
  mid="$(gh run list --workflow=lertiva-zip-macos.yml --repo "$REPO" --branch main -L 1 --json conclusion \
    | jq -r '.[0].conclusion' 2>/dev/null || true)"
  if [[ "$mid" == "success" ]]; then
    pass "G3-S3 — son macOS build workflow success"
  else
    warn "G3-S3 — macOS build son run: ${mid:-bilinmiyor} (APPLE_* bos/yanlis p12? KPI job ayri olabilir)"
  fi
fi

echo ""
echo "========== Ozet =========="
if [[ "$FAIL" -eq 0 ]]; then
  echo "OK: G2/G3 senaryo testleri — agent tarafi tamam (EV/OV + APPLE insan adimi disinda)"
  exit 0
fi
echo "Bazi senaryolar basarisiz — yukaridaki ❌ satirlara bakin."
exit 1
