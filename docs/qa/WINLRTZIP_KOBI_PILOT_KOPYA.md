# winlrtZIP — KOBİ pilot (e-posta eki, ≈20 dk)

**Kurulum:** [winlrt-zip-beta.1](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-zip-beta.1)

Başlangıç scripti: `scripts/windows/pilot-winlrtzip-kobi-start.ps1`

## 0. Preflight

```powershell
powershell -ExecutionPolicy Bypass -File scripts/windows/winlrtzip-kobi-pilot-preflight.ps1
```

PASS: `email-size-cap=OK` (ZIP ≤ 10 MB).

## 1. Gönderen (KOBİ)

- [ ] Örnek **FaturaPaketi** klasörü (birkaç PDF/Excel veya test dosyası)
- [ ] Explorer **winlrtZIP — hızlı ZIP** *veya* uygulama içi **e-posta profili** ile `.zip`
- [ ] Oluşan `.zip` **10 MB altında** (Outlook/Gmail eki limitine uygun)

## 2. Alıcı (aynı makine simülasyonu yeterli)

- [ ] **winlrtZIP ile aç** — liste doğru
- [ ] Çıkar / test — dosyalar bütün

## 3. İsteğe bağlı

- [ ] [WINLRTZIP_EXPLORER_QA_KOPYA.md](./WINLRTZIP_EXPLORER_QA_KOPYA.md) I1/I2 tamamlandı mı?

**Bitti mi?** Chat: **winlrtZIP KOBİ pilot bitti** + OS + SHA256 + PASS/FAIL → agent signoff → commit · push · deploy.
