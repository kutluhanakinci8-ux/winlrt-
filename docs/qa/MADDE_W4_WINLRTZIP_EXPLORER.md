# Madde W4 — Lertiva Zip Explorer sağ tık %100

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| NSIS hook | `apps/lertiva-zip/src-tauri/windows/installer-hooks.nsh` |
| CLI / startup | `apps/lertiva-zip/src-tauri/src/cli.rs` |
| UI dinleyiciler | `apps/lertiva-zip/src/App.tsx` |
| Tauri NSIS | `tauri.conf.json` → `bundle.windows.nsis.installerHooks` |

## Menüler (`.zip` + klasör + çoklu seçim)

| Menü | Davranış |
|------|----------|
| Lertiva Zip ile aç | UI’da arşiv listesi |
| Lertiva Zip — buraya aç | Headless çıkarma (komşu klasör) |
| Lertiva Zip ile sıkıştır | UI: `KlasorAdi.zip` (seçili profil) |
| Lertiva Zip — hızlı ZIP | Headless e-posta profili |
| Lertiva Zip — seçilenleri sıkıştır | UI: çoklu dosya → `.zip` |

Şifreli sıkıştırma: [MADDE_W5](./MADDE_W5_WINLRTZIP_PASSWORD.md).

## W4 %100

1. `installer-hooks.nsh` + `cli.rs` + `App.tsx` event’leri repo’da
2. `bash scripts/verify-madde-w4.sh`
3. Sonraki winlrt-zip-beta NSIS kurulumu Explorer menülerini yazar (CI run sonrası)
