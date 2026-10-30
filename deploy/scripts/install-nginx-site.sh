#!/usr/bin/env bash
# nginx site config — repo kökünden veya /var/www/winlrt içinden çalıştırın.
set -euo pipefail

DOMAIN="${WINLRT_DOMAIN:-winlrt.com.tr}"
WEB_ROOT="${WINLRT_WEB_ROOT:-/var/www/winlrt/apps/web/dist}"
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TEMPLATE="${REPO_ROOT}/deploy/nginx/winlrt.conf.template"
SITE="/etc/nginx/sites-available/winlrt.conf"

if [[ ! -f "$TEMPLATE" ]]; then
  echo "Şablon bulunamadı: $TEMPLATE" >&2
  exit 1
fi

mkdir -p "$WEB_ROOT"
sed -e "s|__WINLRT_DOMAIN__|${DOMAIN}|g" -e "s|__WINLRT_WEB_ROOT__|${WEB_ROOT}|g" \
  "$TEMPLATE" > /tmp/winlrt.nginx.conf

if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
  sudo cp /tmp/winlrt.nginx.conf "$SITE"
  sudo ln -sf "$SITE" /etc/nginx/sites-enabled/winlrt.conf
  sudo rm -f /etc/nginx/sites-enabled/default
  sudo nginx -t
  sudo systemctl reload nginx
else
  cp /tmp/winlrt.nginx.conf "$SITE"
  ln -sf "$SITE" /etc/nginx/sites-enabled/winlrt.conf
  rm -f /etc/nginx/sites-enabled/default
  nginx -t
  systemctl reload nginx
fi

echo "OK: nginx winlrt → ${DOMAIN} root ${WEB_ROOT}"
