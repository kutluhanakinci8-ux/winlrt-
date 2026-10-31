# Faz G4 — Masaüstü olgunluk (repo kapısı)

| Kapı | Doğrulama |
|------|-----------|
| Explorer QA kaydı | `docs/qa/EXPLORER_QA_SIGNOFF.md` PASS |
| Kurumsal pilot kaydı | `docs/qa/KURUMSAL_PILOT_SIGNOFF.md` PASS |
| Kurumsal paket | `verify-dalga3-prereqs.sh` |
| v1 signoff script | `check-v1-gate-ready.sh` |
| Zip + Arc derleme | `cargo check -p lertiva-zip`, `cargo check -p lertiva-arc` |

```bash
bash scripts/verify-faz-g4.sh
```

**Ürün %100:** en az 1 dış kullanıcı oturumu + Dalga 5 pilot (insan).
