#!/usr/bin/env bash
# Mac: TAURI_SIGNING_* GitHub Secrets (PFX veya Tauri signer key dosyası).
# Kullanım:
#   bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/cert.pfx
#   bash scripts/mac-set-github-tauri-signing-secrets.sh ~/.tauri/winlrt-rar.key
set -euo pipefail

REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
KEY_FILE="${1:-}"

if [[ -z "$KEY_FILE" || ! -f "$KEY_FILE" ]]; then
  echo "Kullanım: $0 /yol/imza-dosyasi (.pfx veya Tauri private key)"
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "gh yok — önce: bash scripts/mac-install-gh-portable.sh && ~/.local/bin/gh auth login"
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  gh auth login -h github.com -p https -w
fi

case "$KEY_FILE" in
  *.pfx|*.p12)
    echo "PFX → Base64 tek satır secret (Tauri CI örneği)."
    base64 <"$KEY_FILE" | tr -d '\n' | gh secret set TAURI_SIGNING_PRIVATE_KEY --repo "$REPO"
    ;;
  *)
    gh secret set TAURI_SIGNING_PRIVATE_KEY --repo "$REPO" <"$KEY_FILE"
    ;;
esac

echo "Parola secret (ekranda görünmez):"
gh secret set TAURI_SIGNING_PRIVATE_KEY_PASSWORD --repo "$REPO"
echo "Tamam: TAURI_SIGNING_* kaydedildi ($REPO)."
echo "Actions → Lertiva Arc Windows beta build → Run workflow"
