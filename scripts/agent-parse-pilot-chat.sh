#!/usr/bin/env bash
# Chat yapıştırması → signoff script (Dalga 2 veya 3).
# Usage: bash scripts/agent-parse-pilot-chat.sh <<'EOF' ... EOF
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TEXT="$(cat)"

if echo "$TEXT" | grep -qi 'QA bitti'; then
  os="$(echo "$TEXT" | sed -n 's/^[Oo][Ss][[:space:]]*:[[:space:]]*//p' | head -1)"
  sha="$(echo "$TEXT" | sed -n 's/^[Ss][Hh][Aa]256[[:space:]]*:[[:space:]]*//p' | head -1)"
  by="$(echo "$TEXT" | sed -n 's/^QA yapan[[:space:]]*:[[:space:]]*//p' | head -1)"
  note="$(echo "$TEXT" | sed -n 's/^[Nn]ot[[:space:]]*:[[:space:]]*//p' | head -1)"
  : "${os:?OS satırı gerekli (OS: ...)}"
  : "${sha:?SHA256 satırı gerekli}"
  : "${by:?QA yapan satırı gerekli}"
  EXPLORER_QA_OS="$os" EXPLORER_QA_SHA256="$sha" EXPLORER_QA_BY="$by" EXPLORER_QA_NOTES="$note" \
    bash "$ROOT/scripts/agent-explorer-signoff.sh"
  exit 0
fi

if echo "$TEXT" | grep -qi 'kurumsal pilot bitti'; then
  if ! grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" 2>/dev/null; then
    echo "Explorer signoff PASS değil — önce Dalga 2." >&2
    exit 1
  fi
  seg="$(echo "$TEXT" | sed -n 's/^[Ss]egment[[:space:]]*:[[:space:]]*//p' | head -1)"
  pol="$(echo "$TEXT" | sed -n 's/^[Pp]olitika[[:space:]]*:[[:space:]]*//p' | head -1)"
  aud="$(echo "$TEXT" | sed -n 's/^[Dd]enetim[[:space:]]*:[[:space:]]*//p' | head -1)"
  by="$(echo "$TEXT" | sed -n 's/^[Oo]naylayan[[:space:]]*:[[:space:]]*//p' | head -1)"
  : "${seg:?Segment gerekli}"
  : "${pol:?Politika gerekli}"
  : "${by:?Onaylayan gerekli}"
  CORP_PILOT_SEGMENT="$seg" CORP_PILOT_POLICY="$pol" CORP_PILOT_AUDIT="${aud:-test edildi}" CORP_PILOT_BY="$by" \
    bash "$ROOT/scripts/agent-corporate-signoff.sh"
  exit 0
fi

echo "Tanınmayan chat — bkz. docs/qa/PILOT_CHAT_SABLONLARI.md" >&2
exit 1
