# Dalga 2 — tek oturum pilot (≈30 dk)

**Sıra:** Kur → preflight → checklist → chat.

| Adım | Ne yapılır |
|------|------------|
| 1 | [Beta setup](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814) kur |
| 2 | `scripts/windows/explorer-qa-preflight.ps1` — **SHA256** kopyala |
| 3 | [DALGA2_EXPLORER_QA_KOPYA.md](./DALGA2_EXPLORER_QA_KOPYA.md) tüm kutular |
| 4 | Chat: **QA bitti** + SHA256 + OS sürümü + varsa FAIL notu |

Agent: `scripts/agent-explorer-signoff.sh` (ortam: `EXPLORER_QA_OS`, `EXPLORER_QA_SHA256`, `EXPLORER_QA_BY`) → commit · push · deploy → **Dalga 3**.

**Dalga 3 hazır:** [DALGA3_KURUMSAL_KOPYA.md](./DALGA3_KURUMSAL_KOPYA.md) (Explorer PASS sonrası).
