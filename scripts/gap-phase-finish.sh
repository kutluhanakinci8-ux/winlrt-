#!/usr/bin/env bash
# Faz sonu: verify → gate JSON → gap raporu → (opsiyonel) commit + vitrin deploy.
# Kullanım: bash scripts/gap-phase-finish.sh 4 [--commit "mesaj"] [--deploy]
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PHASE="${1:?Faz numarası gerekli (ör. 4)}"
shift || true
COMMIT_MSG=""
DO_DEPLOY=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit)
      COMMIT_MSG="${2:?--commit mesaj gerekli}"
      shift 2
      ;;
    --deploy) DO_DEPLOY=true; shift ;;
    *) echo "Bilinmeyen: $1" >&2; exit 1 ;;
  esac
done

bash "$ROOT/scripts/verify-faz-g${PHASE}.sh"
case "$PHASE" in
  4) bash "$ROOT/scripts/evaluate-lertiva-desktop-maturity-gate.sh" ;;
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
  bash "$ROOT/scripts/post-phase-deploy.sh"
fi

echo "OK: gap-phase-finish G${PHASE}"
