#!/usr/bin/env bash
# Madde E+F: VPS üzerinde git-pull deploy (agent SSH; GitHub secret 403 bypass).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOST="${VPS_HOST:-37.230.105.99}"
PORT="${VPS_PORT:-23422}"
USER="${VPS_USER:-root}"
PASS="${VPS_SSH_PASSWORD:?VPS_SSH_PASSWORD}"

SSH=(sshpass -p "$PASS" ssh -o StrictHostKeyChecking=accept-new -p "$PORT" "${USER}@${HOST}")

"${SSH[@]}" "mkdir -p /var/www/winlrt/scripts"
sshpass -p "$PASS" scp -o StrictHostKeyChecking=accept-new -P "$PORT" \
  "$ROOT/scripts/vps-vitrin-sync-from-main.sh" "${USER}@${HOST}:/var/www/winlrt/scripts/"

"${SSH[@]}" 'bash -s' <<'REMOTE'
set -euo pipefail
command -v zip >/dev/null || apt-get update -qq && apt-get install -y -qq zip
install -m 755 /var/www/winlrt/scripts/vps-vitrin-sync-from-main.sh /usr/local/bin/winlrt-vitrin-sync.sh
cat >/etc/cron.d/winlrt-vitrin-sync <<'CRON'
# winlrt vitrin — main sync (Madde E/F)
*/10 * * * * root /usr/local/bin/winlrt-vitrin-sync.sh >>/var/log/winlrt-vitrin-sync.log 2>&1
CRON
chmod 644 /etc/cron.d/winlrt-vitrin-sync
/usr/local/bin/winlrt-vitrin-sync.sh
REMOTE

echo "OK: VPS cron deploy kuruldu"
