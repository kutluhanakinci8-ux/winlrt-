#!/usr/bin/env bash
# Faz sonu: verify → gate JSON → gap raporu → commit (ops.) → vitrin deploy (VPS varsa otomatik).
# Kullanım: bash scripts/gap-phase-finish.sh 4 [--commit "mesaj"] [--deploy] [--no-deploy]
# VPS: VPS_SSH_PASSWORD / VPS_SSH_KEY* ortamda veya .env.vps.local (bkz. docs/env-vps-local.example)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
# shellcheck source=scripts/lib/load-vps-deploy-env.sh
source "$ROOT/scripts/lib/load-vps-deploy-env.sh"

PHASE="${1:?Faz numarası gerekli (ör. 4)}"
shift || true
COMMIT_MSG=""
DO_DEPLOY=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit)
      COMMIT_MSG="${2:?--commit mesaj gerekli}"
      shift 2
      ;;
    --deploy) DO_DEPLOY=true; shift ;;
    --no-deploy) DO_DEPLOY=false; shift ;;
    *) echo "Bilinmeyen: $1" >&2; exit 1 ;;
  esac
done
if [[ "$DO_DEPLOY" == "" ]]; then
  if vps_deploy_ready; then
    DO_DEPLOY=true
  else
    DO_DEPLOY=false
  fi
fi

bash "$ROOT/scripts/verify-faz-g${PHASE}.sh"
case "$PHASE" in
  4) bash "$ROOT/scripts/evaluate-lertiva-desktop-maturity-gate.sh" ;;
  5) bash "$ROOT/scripts/evaluate-lertiva-corporate-gate.sh" ;;
  8) bash "$ROOT/scripts/evaluate-lertiva-lerta-staging-gate.sh" ;;
  9) bash "$ROOT/scripts/evaluate-lertiva-motor-advanced-gate.sh" ;;
esac
bash "$ROOT/scripts/evaluate-gap-status-report.sh"

if [[ -n "$COMMIT_MSG" ]]; then
  git add -A
  if git diff --staged --quiet; then
    echo "Commit yok (değişiklik yok)."
  else
    git commit -m "$COMMIT_MSG"
  fi
fi

if [[ "$DO_DEPLOY" == true ]]; then
  if ! vps_deploy_ready; then
    echo "Deploy atlandi: VPS_SSH_PASSWORD / VPS_SSH_KEY* veya .env.vps.local yok." >&2
    echo "GitHub Actions: repo Secrets → VPS_SSH_PASSWORD (docs/AUTO_DEPLOY.md)" >&2
    exit 1
  fi
  bash "$ROOT/scripts/post-phase-deploy.sh"
else
  echo "Deploy atlandi (--no-deploy veya VPS kimligi yok)."
fi

echo "OK: gap-phase-finish G${PHASE}"
