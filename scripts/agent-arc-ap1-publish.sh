#!/usr/bin/env bash
# AP-1 — imzalı v1.0.2 stable release (EV/OV secret zorunlu).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TAG="${1:-v1.0.2}"

echo "==> Madde M / imza prep"
bash "$ROOT/scripts/verify-madde-m.sh"

REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
if ! gh secret list --repo "$REPO" 2>/dev/null | grep -qE 'WINDOWS_CERTIFICATE|TAURI_SIGNING_PRIVATE_KEY'; then
  echo "Hata: GitHub secret yok — önce AP1 runbook Bölüm 1 (PFX)." >&2
  echo "  bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/kurumsal.pfx" >&2
  exit 1
fi

echo "==> Tag push → lertiva-arc-windows-stable"
bash "$ROOT/scripts/publish-lertiva-arc-stable-tag.sh" "$TAG"

echo "==> CI bitince:"
echo "  1) Release asset imzasını doğrulayın (Windows: verify-windows-signature.ps1)"
echo "  2) apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag içeriği: ev_ov"
echo "  3) bash scripts/refresh-lertiva-downloads-manifest.sh && deploy"
echo "  4) bash scripts/evaluate-lertiva-arc-platform-gate.sh  → AP-1 pass"
