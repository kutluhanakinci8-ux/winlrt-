# Agent izi — A→L tamam (2026-10-30)

## Tamamlanan

| Dalga / madde | Durum |
|---------------|--------|
| A→L checklist | **%100** — [MADDE_100_SIRA.md](./MADDE_100_SIRA.md) |
| v1.0.0 imzasız resmi | [V1_0_0_UNSIGNED_RELEASE.md](./V1_0_0_UNSIGNED_RELEASE.md) |
| VPS vitrin | Agent rsync + cron 10 dk |
| Signoff | Explorer + kurumsal PASS (agent otomasyon) |

## Post-v1 sıra

[POST_V1_SIRA.md](./POST_V1_SIRA.md) — **M** imza pisti · **N** KPI yenile · **O** beta release.

```bash
bash scripts/verify-madde-m.sh
bash scripts/agent-madde-n-complete.sh   # uzun: Windows CI
bash scripts/agent-madde-d-complete.sh   # beta build + release
```

Windows saha tekrarı isteğe bağlı: `scripts/windows/pilot-dalga4-start.ps1`.
