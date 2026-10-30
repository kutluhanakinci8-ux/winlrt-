# Dalga 4 — v1.0 kapı oturumu

**Önkoşullar:** [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) PASS · [KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) PASS.

| Kapı | Kanıt |
|------|--------|
| Windows KPI | Vitrin `windows-*-kpi.json` + resmi bench artifact |
| Explorer QA | Signoff dosyası PASS |
| Kurumsal pilot | Signoff dosyası PASS |
| İmzalı kurulum | `TAURI_SIGNING_*` (veya imzasız beta + dokümante istisna) |

**Agent kontrol:** `bash scripts/check-v1-gate-status.sh`

**Kapanış:** Tag `v1.0.0`, release notes, vitrin duyurusu → agent commit · push · deploy.

Detay: [FAZ6_V1_KAPOR.md](./FAZ6_V1_KAPOR.md) · [WINLRTRAR_PRODUCTION_CHECKLIST.md](../WINLRTRAR_PRODUCTION_CHECKLIST.md) §7.
