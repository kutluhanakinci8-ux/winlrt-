#!/usr/bin/env bash
# shellcheck disable=SC2034
# Kullanim: source "$ROOT/scripts/lib/gh-usable.sh"
gh_usable=false
if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
  gh_usable=true
fi
