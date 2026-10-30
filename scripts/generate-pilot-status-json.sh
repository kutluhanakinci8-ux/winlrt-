#!/usr/bin/env bash
# Vitrin: aktif dalga + signoff durumu (build sırasında).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/pilot/status.json"
mkdir -p "$(dirname "$OUT")"

explorer="pending"
corp="pending"
if grep -q '\[x\] PASS' "$ROOT/docs/qa/EXPLORER_QA_SIGNOFF.md" 2>/dev/null; then
  explorer="pass"
fi
if grep -q '\[x\] PASS' "$ROOT/docs/qa/KURUMSAL_PILOT_SIGNOFF.md" 2>/dev/null; then
  corp="pass"
fi

wave=2
if [ "$explorer" = "pass" ] && [ "$corp" = "pass" ]; then
  wave=4
elif [ "$explorer" = "pass" ]; then
  wave=3
fi

generated="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

cat >"$OUT" <<EOF
{
  "generated_at": "${generated}",
  "active_wave": ${wave},
  "waves": {
    "1_deploy": "pass",
    "2_explorer_qa": "${explorer}",
    "3_corporate": "${corp}",
    "4_v1": "pending"
  },
  "links": {
    "beta_release": "https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814",
    "dalga2_doc": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/DALGA2_PILOT_OTURUM.md",
    "corporate_zip": "/downloads/winlrtRAR-corporate-gpo.zip",
    "chat_templates": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/PILOT_CHAT_SABLONLARI.md",
    "dalga4_doc": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/docs/qa/DALGA4_V1_OTURUM.md"
  },
  "scripts": {
    "dalga2": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/scripts/windows/pilot-dalga2-start.ps1",
    "dalga3": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/scripts/windows/pilot-dalga3-start.ps1",
    "dalga4": "https://github.com/kutluhanakinci8-ux/winlrt-/blob/main/scripts/windows/pilot-dalga4-start.ps1"
  }
}
EOF

echo "OK: $OUT (active_wave=${wave})"
