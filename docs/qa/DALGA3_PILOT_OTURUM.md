# Dalga 3 — kurumsal pilot oturumu (≈45 dk)

**Önkoşul:** [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) PASS.

| Adım | Ne yapılır |
|------|------------|
| 1 | `scripts/windows/pilot-dalga3-start.ps1` **veya** [Lertiva Arc-corporate-gpo.zip](https://winlrt.com.tr/downloads/Lertiva Arc-corporate-gpo.zip) |
| 2 | ADMX/ADML → PolicyDefinitions **veya** `apply-corporate-sample.ps1` (test PC) |
| 3 | `corporate-pilot-preflight.ps1` — `PasswordPolicy=corporate` |
| 4 | [DALGA3_KURUMSAL_KOPYA.md](./DALGA3_KURUMSAL_KOPYA.md) senaryolar |
| 5 | Chat: **kurumsal pilot bitti** + segment + PASS/FAIL |

Agent: [KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) → commit · push · deploy → **Dalga 4 / Faz 6**.
