#!/usr/bin/env bash
# Turhost Ubuntu — ilk kurulum (root veya sudo). DNS A kaydı sunucuya işaret etmeli.
set -euo pipefail

DOMAIN="${WINLRT_DOMAIN:-winlrt.com.tr}"
WEB_ROOT="${WINLRT_WEB_ROOT:-/var/www/winlrt/apps/web/dist}"
REPO_URL="${WINLRT_REPO:-git@github.com:kutluhanakinci8-ux/winlrt-.git}"
APP_DIR="${WINLRT_APP_DIR:-/var/www/winlrt}"

run_root() {
  if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
    sudo "$@"
  else
    "$@"
  fi
}

echo "==> Paketler (nginx, certbot, node 20, git)"
run_root apt-get update -qq
run_root apt-get upgrade -y
run_root apt-get install -y git curl nginx certbot python3-certbot-nginx ufw

run_root ufw allow OpenSSH
run_root ufw allow "Nginx Full"
run_root ufw --force enable

if ! command -v node >/dev/null || [[ "$(node -v | cut -d. -f1 | tr -d v)" -lt 20 ]]; then
  curl -fsSL https://deb.nodesource.com/setup_20.x | run_root bash -
  run_root apt-get install -y nodejs
fi

echo "==> Uygulama dizini: ${APP_DIR}"
run_root mkdir -p "$(dirname "$APP_DIR")"
if [[ ! -d "${APP_DIR}/.git" ]]; then
  run_root git clone "$REPO_URL" "$APP_DIR"
fi

cd "$APP_DIR"
npm ci
npm run build:web

export WINLRT_DOMAIN="$DOMAIN"
export WINLRT_WEB_ROOT="$WEB_ROOT"
bash "${APP_DIR}/deploy/scripts/install-nginx-site.sh"

echo "==> SSL (domain DNS bu sunucuya geliyorsa)"
echo "    certbot --nginx -d ${DOMAIN}"
echo ""
echo "Sonraki adım: GitHub Actions secrets + deploy (bkz. docs/CANLI_SITE.md)"
