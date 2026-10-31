#!/usr/bin/env bash
# Mac/Linux — G2/G3 scriptleri icin arac kontrolu.
set -euo pipefail
MISS=0
need() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Eksik: $1 — $2"
    MISS=1
  fi
}
need bash "bash"
need openssl "openssl (genelde macOS'ta var)"
need jq "brew install jq"
need grep "grep"

if ! command -v gh >/dev/null 2>&1; then
  echo "Opsiyonel: gh — brew install gh && gh auth login (CI senaryolari icin)"
elif ! gh auth status >/dev/null 2>&1; then
  echo "Opsiyonel: gh auth login (CI log senaryolari icin)"
fi

if [[ "$MISS" -ne 0 ]]; then
  echo ""
  echo "Ornek (Homebrew): brew install jq"
  exit 1
fi
echo "OK: jq + openssl hazir"
