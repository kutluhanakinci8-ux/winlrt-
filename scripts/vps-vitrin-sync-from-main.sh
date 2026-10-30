#!/usr/bin/env bash
# VPS: origin/main → vitrin build → WEB_ROOT (Madde E/F — GitHub secret olmadan).
set -euo pipefail
REPO_DIR="${WINLRT_REPO_DIR:-/var/www/winlrt}"
WEB_ROOT="${VPS_WEB_ROOT:-/var/www/winlrt/apps/web/dist}"
cd "$REPO_DIR"
git fetch origin main
git reset --hard origin/main
npm ci
npm run build:web
rsync -av --delete apps/web/dist/ "$WEB_ROOT/"
echo "OK: vitrin sync $(date -u +%Y-%m-%dT%H:%MZ) @ $(git rev-parse --short HEAD)"
