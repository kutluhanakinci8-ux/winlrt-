# Dalga 2 — Explorer QA (Windows pilot kopyala-işaretle)

**Kurulum:** https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.41064653814  

Hızlı akış: [DALGA2_PILOT_OTURUM.md](./DALGA2_PILOT_OTURUM.md) · Tam liste: [WINLRTRAR_EXPLORER_QA.md](../WINLRTRAR_EXPLORER_QA.md)

## 0. Preflight (kurulumdan hemen sonra)

PowerShell (repo veya kopyalanmış script):

```powershell
powershell -ExecutionPolicy Bypass -File scripts/windows/explorer-qa-preflight.ps1
```

PASS ise aşağıdaki manuel maddelere geçin. Çıktıdaki **SHA256** değerini signoff tablosuna yapıştırın.

## Kurulum
- [ ] NSIS setup kuruldu
- [ ] Kaldır-yeniden-kur menü çiftlenmedi

## Arşiv (zip / wlrt / rar / 7z)
- [ ] winlrtRAR ile aç
- [ ] buraya aç
- [ ] Parolalı rar/7z (bilinen sınırlama)

## Klasör
- [ ] sıkıştır
- [ ] hızlı .wlrt
- [ ] şifreli sıkıştır

## Çoklu seçim
- [ ] Aynı klasör stem
- [ ] Farklı klasörler
- [ ] seçilenler şifreli + audit

## Temizlik
- [ ] Kaldırma → Explorer menüleri gitti

**Bitti mi?** Chat’e “QA bitti” + notlar → agent signoff commit · push · deploy.
