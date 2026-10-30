#!/usr/bin/env bash
set -euo pipefail
HOST="${VPS_HOST:-37.230.105.99}"
PORT="${VPS_PORT:-23422}"
USER="${VPS_USER:-root}"
PASS="${VPS_SSH_PASSWORD:?VPS_SSH_PASSWORD}"

SSH=(sshpass -p "$PASS" ssh -o StrictHostKeyChecking=accept-new -p "$PORT" "${USER}@${HOST}")

"${SSH[@]}" test -x /usr/local/bin/winlrt-vitrin-sync.sh
"${SSH[@]}" test -f /etc/cron.d/winlrt-vitrin-sync

LOCAL="$(git -C "$(cd "$(dirname "$0")/.." && pwd)" rev-parse HEAD)"
REMOTE="$("${SSH[@]}" "git -C /var/www/winlrt rev-parse HEAD")"
# Cron birkaç dk gecikebilir; en az script ve cron var
echo "OK: Madde E+F — VPS cron deploy (local=$LOCAL remote=$REMOTE)"
