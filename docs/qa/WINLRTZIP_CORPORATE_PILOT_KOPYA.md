# Lertiva Zip — kurumsal pilot (≈30 dk)

Paket: [lertiva-zip-corporate-gpo.zip](/downloads/lertiva-zip-corporate-gpo.zip) veya repo `deploy/gpo/`

## 0. Preflight

```powershell
powershell -ExecutionPolicy Bypass -File scripts/windows/lertiva-zip-corporate-pilot-preflight.ps1
```

## 1. Politika

- [ ] ADMX kurulumu veya `lertiva-zip-corporate-policy-sample.reg` import
- [ ] `PasswordPolicy=corporate`, `RequireModernCrypto=1`

## 2. Uygulama

- [ ] Lertiva Zip kurulu — şifreli ZIP oluştur (AES) — zayıf parola reddedilmeli
- [ ] Çıkar / oluştur sonrası `%APPDATA%\Lertiva Zip\audit.jsonl` satırı (parolasız)

## 3. Denetim (isteğe)

- [ ] `AuditMirrorPath` hedefinde yeni satır
- [ ] `AuditForwardUrl` test toplayıcısı (HTTP 2xx)

**Bitti mi?** Chat: **Lertiva Zip kurumsal pilot bitti** + segment + PASS → `scripts/agent-lertiva-zip-corporate-signoff.sh` → commit · push · deploy.

Signoff şablonu: [WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md](./WINLRTZIP_CORPORATE_PILOT_SIGNOFF.md) · [MADDE_W19](./MADDE_W19_WINLRTZIP_CORPORATE_SIGNOFF.md)
