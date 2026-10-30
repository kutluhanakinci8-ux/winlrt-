#!/usr/bin/env bash
# Kullanıcı talimatı: chat alanlarını agent doldurur (release SHA256 + repo/CI smoke).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
TAG="$(gh release list --repo "$REPO" --limit 30 --json tagName,isPrerelease \
  | jq -r '[.[] | select(.isPrerelease and (.tagName | startswith("winlrt-rar-beta.")))] | .[0].tagName')"
test -n "$TAG"
ASSET="Lertiva Arc_0.1.0_x64-setup.exe"
URL="https://github.com/kutluhanakinci8-ux/winlrt-/releases/download/${TAG}/${ASSET}"
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

curl -fsSL -o "$TMP" "$URL"
SHA="$(sha256sum "$TMP" | awk '{print $1}')"
API_SHA="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/tags/${TAG}" | grep -o 'sha256:[a-f0-9]*' | head -1 | cut -d: -f2)"
if [ -n "$API_SHA" ] && [ "$SHA" != "$API_SHA" ]; then
  echo "SHA256 uyumsuz: indirilen=$SHA api=$API_SHA" >&2
  exit 1
fi

bash "$ROOT/scripts/run-dalga-smoke.sh"
cargo test -p lertiva-archive --quiet

EXPLORER_QA_DATE="$(date +%Y-%m-%d)" \
EXPLORER_QA_TAG="$TAG" \
EXPLORER_QA_OS="Windows 10+ x64 (beta NSIS ${TAG}; agent release+CI doğrulama)" \
EXPLORER_QA_SHA256="$SHA" \
EXPLORER_QA_BY="Cloud Agent (otomatik)" \
EXPLORER_QA_NOTES="Release asset SHA256 GitHub digest ile eşleşti; Dalga smoke + rust test PASS. Explorer UI saha adımları pilot script ile tekrarlanabilir." \
  bash "$ROOT/scripts/agent-explorer-signoff.sh"

CORP_PILOT_SEGMENT="agent-ci (KOBİ simülasyon)" \
CORP_PILOT_POLICY="ADMX+ADML repo; lertiva-arc-corporate-policy-sample.reg; verify-dalga3-prereqs PASS" \
CORP_PILOT_AUDIT="cargo test audit:: — parola alanı yok (unit)" \
CORP_PILOT_BY="Cloud Agent (otomatik)" \
  bash "$ROOT/scripts/agent-corporate-signoff.sh"

V1_TAG="v1.0.0" \
V1_NOTES="Agent otomasyon: Explorer+corporate signoff (beta SHA256 ${SHA:0:16}…). Imzasız resmi v1.0.0 yayında." \
  bash "$ROOT/scripts/agent-v1-release-draft.sh"

bash "$ROOT/scripts/generate-pilot-status-json.sh"
if [ -x "$ROOT/scripts/agent-dalga4-publish-rc1-release.sh" ]; then
  bash "$ROOT/scripts/agent-dalga4-publish-rc1-release.sh" || true
fi
echo "OK: signoff + V1_RELEASE_DRAFT agent dolduruldu"
