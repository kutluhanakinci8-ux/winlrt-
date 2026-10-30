# Faz 5 — Kurumsal pilot (sıra)

**Önkoşul:** Faz 4 Explorer QA signoff ([EXPLORER_QA_SIGNOFF.md](./EXPLORER_QA_SIGNOFF.md)).

## 1. Segment seçin

[WINLRTRAR_PILOT_KICKOFF.md](../WINLRTRAR_PILOT_KICKOFF.md) — banka / sendika / KOBİ.

## 2. Politika

| Yol | Dosya |
|-----|--------|
| ADMX | `deploy/gpo/Lertiva Arc.admx` → `C:\Windows\PolicyDefinitions\` |
| Örnek .reg | `scripts/winlrt-corporate-policy-sample.reg` |

Önerilen: `PasswordPolicy=corporate`, `RequireModernCrypto=1`, isteğe audit mirror URL.

## 3. Senaryolar

1. Zayıf parola → red (`--secure` / şifreli sıkıştır)
2. Güçlü parola → şifreli `.wlrt` veya ZIP AES
3. Audit: `audit.jsonl` satırı + isteğe HTTP forward

## 4. Kayıt

Chat’e kurumsal pilot sonucu → agent [KURUMSAL_PILOT_SIGNOFF.md](./KURUMSAL_PILOT_SIGNOFF.md) commit · push · deploy.

## 5. v1.0 kapısı (Faz 6)

Windows KPI vitrin + imzalı release (isteğe) + bu pilot onayı → tag `v1.0.0`.
