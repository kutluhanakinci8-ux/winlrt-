#!/usr/bin/env bash
# Sertifika var ama 443 yok / certbot "www.winlrt.com.tr server block" hatası — sunucuda root ile çalıştırın.
set -euo pipefail

DOMAIN="${WINLRT_DOMAIN:-winlrt.com.tr}"
CONF="/etc/nginx/sites-enabled/winlrt.conf"

if [[ ! -f "$CONF" ]]; then
  echo "Bulunamadı: $CONF" >&2
  exit 1
fi

if ! grep -q "www.${DOMAIN}" "$CONF"; then
  sed -i "s/server_name ${DOMAIN};/server_name ${DOMAIN} www.${DOMAIN};/g" "$CONF"
  sed -i "s/server_name ${DOMAIN} /server_name ${DOMAIN} www.${DOMAIN} /g" "$CONF"
fi

nginx -t
certbot install --cert-name "${DOMAIN}" --nginx --redirect
systemctl reload nginx

echo "OK: dinlenen portlar:"
ss -tlnp | grep -E ':80|:443' || true
