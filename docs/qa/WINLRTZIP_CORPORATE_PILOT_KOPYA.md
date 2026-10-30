# winlrtZIP — kurumsal pilot (≈30 dk)

Paket: [winlrtZIP-corporate-gpo.zip](/downloads/winlrtZIP-corporate-gpo.zip) veya repo `deploy/gpo/`

## 0. Preflight

```powershell
powershell -ExecutionPolicy Bypass -File scripts/windows/winlrtzip-corporate-pilot-preflight.ps1
```

## 1. Politika

- [ ] ADMX kurulumu veya `winlrtzip-corporate-policy-sample.reg` import
- [ ] `PasswordPolicy=corporate`, `RequireModernCrypto=1`

## 2. Uygulama

- [ ] winlrtZIP kurulu — şifreli ZIP oluştur (AES) — zayıf parola reddedilmeli
- [ ] Çıkar / oluştur sonrası `%APPDATA%\winlrtZIP\audit.jsonl` satırı (parolasız)

## 3. Denetim (isteğe)

- [ ] `AuditMirrorPath` hedefinde yeni satır
- [ ] `AuditForwardUrl` test toplayıcısı (HTTP 2xx)

**Bitti mi?** Chat: **winlrtZIP kurumsal pilot bitti** → agent signoff (isteğe).
