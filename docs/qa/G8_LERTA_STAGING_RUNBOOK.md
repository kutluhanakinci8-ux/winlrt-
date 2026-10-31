# G8 — Lerta staging runbook

**Ertelenen (prod):** `LERTA_API_BEARER` gerçek secret · 1 canlı metadata kaydı (IT + KVKK).

## 1) Staging smoke (agent / CI)

```bash
export WINLRT_LERTA_API_DRY_RUN=1
unset LERTA_API_BEARER
bash scripts/run-lerta-staging-smoke.sh
bash scripts/evaluate-lertiva-lerta-staging-gate.sh
```

Beklenen: `lertiva-lerta-staging-gate.json` → **`repo_pass: true`**, `live_submit_allowed: false`.

## 2) Vitrin JSON

| Kanal | Örnek |
|-------|--------|
| Birleşik | [lertiva-zip-lerta-integration-v1.json](/benchmark/samples/lertiva-zip-lerta-integration-v1.json) |
| Posta W28 | [lertiva-zip-lerta-mail-v1.json](/benchmark/samples/lertiva-zip-lerta-mail-v1.json) |
| API W29 | [lertiva-zip-lerta-api-v1.json](/benchmark/samples/lertiva-zip-lerta-api-v1.json) |
| Mesaj W30 | [lertiva-zip-lerta-message-v1.json](/benchmark/samples/lertiva-zip-lerta-message-v1.json) |

Pano: [/benchmark/lerta/](/benchmark/lerta/)

## 3) Zip CLI (dry-run)

```bash
WINLRT_LERTA_API_DRY_RUN=1 cargo run -p lertiva-archive --bin lertiva-zip -- --json lerta-integration-status
```

## 4) Prod geçiş (IT — repoya secret yazmayın)

1. `docs/env-lerta-prod.example` → güvenli secret store
2. `LERTA_API_BEARER` + `WINLRT_LERTA_API_DRY_RUN=0` (yalnızca onaylı makine)
3. Denetim: [G8_LERTA_AUDIT_LOG.md](./G8_LERTA_AUDIT_LOG.md)
4. `bash scripts/gap-phase-finish.sh 8`

## 5) Kapılar

- Repo: `bash scripts/verify-faz-g8.sh`
- Gap: `bash scripts/gap-phase-finish.sh 8`
