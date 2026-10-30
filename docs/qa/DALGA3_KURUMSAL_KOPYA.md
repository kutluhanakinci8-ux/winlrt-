# Dalga 3 — Kurumsal pilot (Windows, Faz 2 sonrası)

**Önkoşul:** [EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md) PASS.

**Preflight (politika sonrası):** `scripts/windows/corporate-pilot-preflight.ps1`  
**Hızlı akış:** [DALGA3_PILOT_OTURUM.md](./DALGA3_PILOT_OTURUM.md)

## Paket

| Dosya | Kullanım |
|-------|----------|
| [Lertiva Arc-corporate-gpo.zip](https://winlrt.com.tr/downloads/Lertiva Arc-corporate-gpo.zip) | ADMX + ADML + örnek `.reg` |
| `deploy/gpo/Lertiva Arc.admx` | PolicyDefinitions (ZIP içinde) |
| `scripts/winlrt-corporate-policy-sample.reg` | veya `apply-corporate-sample.ps1` (HKLM test) |

## Senaryolar

- [ ] `PasswordPolicy=corporate` — zayıf parola reddi
- [ ] `RequireModernCrypto=1` — zayıf şifreleme reddi
- [ ] Güçlü parola ile şifreli `.wlrt` / ZIP AES
- [ ] Audit: `audit.jsonl` satırı (parola alanı yok)

## Kapanış

[KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) → agent commit · push · deploy.

Rehber: [FAZ5_KURUMSAL_BASLANGIC.md](./FAZ5_KURUMSAL_BASLANGIC.md) · [WINLRTRAR_PILOT_KICKOFF.md](../WINLRTRAR_PILOT_KICKOFF.md)
