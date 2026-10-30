#!/usr/bin/env bash
# Mac: GitHub Secrets kutusuna yapıştırma olmadan VPS_SSH_KEY kaydı.
# Kullanım: bash scripts/mac-set-github-vps-secret.sh
set -euo pipefail

KEY="${HOME}/.ssh/winlrt-gha-deploy"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

if [[ ! -f "$KEY" ]]; then
  echo "Anahtar yok: $KEY"
  echo "Önce: ssh-keygen -t ed25519 -f $KEY -N \"\" -C winlrt-gha-deploy"
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI (gh) yüklü değil. Yükleniyor (Homebrew)..."
  if ! command -v brew >/dev/null 2>&1; then
    echo "Homebrew yok. Önce https://brew.sh adresinden brew kurun, sonra tekrar çalıştırın."
    exit 1
  fi
  brew install gh
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "GitHub oturumu gerekli (tarayıcı açılır, yapıştırma yok):"
  gh auth login -h github.com -p https -w
fi

ssh-keygen -y -f "$KEY" >/dev/null
gh secret set VPS_SSH_KEY --repo "$REPO" <"$KEY"
echo "Tamam: VPS_SSH_KEY kaydedildi ($REPO)."
echo "Actions → Main pipeline → Run workflow"
