# winlrtZIP — Explorer I1/I2 (Windows pilot)

**Kurulum:** [winlrt-zip-beta.1](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-zip-beta.1) (veya güncel winlrt-zip-windows CI çıktısı)

Tam menü listesi: [MADDE_W4](./MADDE_W4_WINLRTZIP_EXPLORER.md)

## 0. Preflight

```powershell
powershell -ExecutionPolicy Bypass -File scripts/windows/winlrtzip-explorer-qa-preflight.ps1
```

PASS ise aşağıya geçin. **SHA256** değerini signoff için saklayın.

## I1 — Explorer ZIP → winlrt

- [ ] Windows Explorer: bir klasörde birkaç dosya seç → **Gönder → Sıkıştırılmış (zip) klasör** ile `.zip` oluştur
- [ ] `.zip` üzerinde **winlrtZIP ile aç** — liste doğru, hata yok
- [ ] **winlrtZIP — buraya aç** — dosyalar komşu klasöre çıkar, yollar doğru

## I2 — winlrt ZIP → Explorer

- [ ] Klasörde **winlrtZIP ile sıkıştır** (veya hızlı ZIP) — `.zip` oluşur
- [ ] Aynı `.zip` üzerinde Explorer **Tümünü çıkar** — uyarı / bozuk arşiv diyaloğu **yok**
- [ ] Çıkan dosyalar kaynakla aynı

## Temizlik (isteğe)

- [ ] Program kaldırma → `.zip` / klasör menüleri gitti

**Bitti mi?** Chat’e “winlrtZIP Explorer QA bitti” + notlar → agent `agent-winlrtzip-explorer-i1-i2-signoff.sh` → commit · push · deploy.
