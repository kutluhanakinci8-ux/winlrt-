# Dalga 3 — Kurumsal pilot (Windows, Faz 2 sonrası)

**Önkoşul:** [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) PASS.

## Paket

| Dosya | Kullanım |
|-------|----------|
| `deploy/gpo/winlrtRAR.admx` | PolicyDefinitions |
| `scripts/winlrt-corporate-policy-sample.reg` | Çift tık veya `reg import` (test OU) |

## Senaryolar

- [ ] `PasswordPolicy=corporate` — zayıf parola reddi
- [ ] `RequireModernCrypto=1` — zayıf şifreleme reddi
- [ ] Güçlü parola ile şifreli `.wlrt` / ZIP AES
- [ ] Audit: `audit.jsonl` satırı (parola alanı yok)

## Kapanış

[KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) → commit → agent deploy.

Rehber: [FAZ5_KURUMSAL_BASLANGIC.md](./FAZ5_KURUMSAL_BASLANGIC.md) · [WINLRTRAR_PILOT_KICKOFF.md](../WINLRTRAR_PILOT_KICKOFF.md)
