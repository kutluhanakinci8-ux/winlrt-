#!/usr/bin/env bash
# G8 — Lerta staging repo kapı özeti (prod bearer ayrı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/apps/web/public/benchmark/samples/lertiva-lerta-staging-gate.json"

export WINLRT_LERTA_API_DRY_RUN=1

w28=false w29=false w30=false w31=false
schemas=false
samples=false
runbook=false
audit_doc=false
prod_example=false
staging_gate=false

bash "$ROOT/scripts/verify-madde-w28.sh" >/dev/null 2>&1 && w28=true
bash "$ROOT/scripts/verify-madde-w29.sh" >/dev/null 2>&1 && w29=true
bash "$ROOT/scripts/verify-madde-w30.sh" >/dev/null 2>&1 && w30=true
bash "$ROOT/scripts/verify-madde-w31.sh" >/dev/null 2>&1 && w31=true

schemas=true
for f in lerta-mail-v1.schema.json lerta-api-v1.schema.json lerta-message-v1.schema.json lerta-integration-v1.schema.json; do
  test -f "$ROOT/docs/qa/$f" || schemas=false
done

bash "$ROOT/scripts/evaluate-lerta-mail-sample.sh" >/dev/null 2>&1
bash "$ROOT/scripts/evaluate-lerta-api-sample.sh" >/dev/null 2>&1
bash "$ROOT/scripts/evaluate-lerta-message-sample.sh" >/dev/null 2>&1
bash "$ROOT/scripts/evaluate-lerta-integration-sample.sh" >/dev/null 2>&1

if [[ -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-mail-v1.json" \
  && -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-api-v1.json" \
  && -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-message-v1.json" \
  && -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-integration-v1.json" ]]; then
  samples=true
fi

test -f "$ROOT/docs/qa/G8_LERTA_STAGING_RUNBOOK.md" && runbook=true
test -f "$ROOT/docs/qa/G8_LERTA_AUDIT_LOG.md" && audit_doc=true
test -f "$ROOT/docs/env-lerta-prod.example" && prod_example=true

if jq -e '.staging_gate_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-integration-v1.json" >/dev/null 2>&1; then
  staging_gate=true
fi

bearer_configured=false
live_submit=false
if jq -e '.bearer_configured == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-integration-v1.json" >/dev/null 2>&1; then
  bearer_configured=true
fi
if jq -e '.live_submit_allowed == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-lerta-integration-v1.json" >/dev/null 2>&1; then
  live_submit=true
fi

repo_pass=false
if [[ "$w28" == true && "$w29" == true && "$w30" == true && "$w31" == true && "$schemas" == true && "$samples" == true && "$runbook" == true && "$audit_doc" == true && "$prod_example" == true && "$staging_gate" == true ]]; then
  repo_pass=true
fi

all_pass=false
if [[ "$repo_pass" == true && "$live_submit" == true ]]; then
  all_pass=true
fi

generated_at="$(date -u +%Y-%m-%dT%H:%MZ)"
jq -n \
  --arg generated_at "$generated_at" \
  --argjson w28 "$([[ "$w28" == true ]] && echo true || echo false)" \
  --argjson w29 "$([[ "$w29" == true ]] && echo true || echo false)" \
  --argjson w30 "$([[ "$w30" == true ]] && echo true || echo false)" \
  --argjson w31 "$([[ "$w31" == true ]] && echo true || echo false)" \
  --argjson schemas "$([[ "$schemas" == true ]] && echo true || echo false)" \
  --argjson samples "$([[ "$samples" == true ]] && echo true || echo false)" \
  --argjson staging_gate "$([[ "$staging_gate" == true ]] && echo true || echo false)" \
  --argjson runbook "$([[ "$runbook" == true ]] && echo true || echo false)" \
  --argjson audit_doc "$([[ "$audit_doc" == true ]] && echo true || echo false)" \
  --argjson prod_example "$([[ "$prod_example" == true ]] && echo true || echo false)" \
  --argjson bearer_configured "$([[ "$bearer_configured" == true ]] && echo true || echo false)" \
  --argjson live_submit "$([[ "$live_submit" == true ]] && echo true || echo false)" \
  --argjson repo_pass "$([[ "$repo_pass" == true ]] && echo true || echo false)" \
  --argjson all_pass "$([[ "$all_pass" == true ]] && echo true || echo false)" \
  '{
    product: "Lertiva Zip ↔ Lerta (G8)",
    generated_at: $generated_at,
    disclaimer: "repo_pass=staging W28–W31; all_pass=LERTA_API_BEARER prod + dry-run kapali (IT).",
    integration_json: "/benchmark/samples/lertiva-zip-lerta-integration-v1.json",
    gates: {
      madde_w28_mail: { pass: $w28 },
      madde_w29_api: { pass: $w29 },
      madde_w30_message: { pass: $w30 },
      madde_w31_integration: { pass: $w31 },
      json_schemas: { pass: $schemas },
      vitrin_samples: { pass: $samples },
      staging_gate_pass: { pass: $staging_gate },
      g8_runbook: { pass: $runbook },
      audit_documentation: { pass: $audit_doc },
      prod_config_example: { pass: $prod_example },
      bearer_configured_vitrin: { pass: $bearer_configured },
      live_submit_allowed: { pass: $live_submit, deferred: true }
    },
    repo_pass: $repo_pass,
    all_pass: $all_pass
  }' >"$OUT"
echo "G8 Lerta — repo: $repo_pass · live: $live_submit · all: $all_pass"
echo "Yazildi: $OUT"
