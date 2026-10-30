# Explorer sağ tık (Faz 3 beta)

Windows NSIS kurulumu sonrası **SystemFileAssociations** altına ek menüler yazılır (varsayılan «Aç» uygulamasını değiştirmez).

| Menü | Komut |
|------|--------|
| Lertiva Arc ile aç | `Lertiva Arc.exe "%1"` |
| Lertiva Arc — buraya aç | `Lertiva Arc.exe --extract-here "%1"` |

## Klasör (Directory)

| Menü | Komut |
|------|--------|
| Lertiva Arc ile sıkıştır | `Lertiva Arc.exe --compress-folder "%1"` (UI: kaynak + önerilen `.wlrt`) |
| Lertiva Arc — şifreli sıkıştır | `Lertiva Arc.exe --compress-folder "%1" --secure` (UI: parola + Oluştur) |
| Lertiva Arc — hızlı .wlrt | `Lertiva Arc.exe --compress-folder-quick "%1"` (Dengeli profil, komşu dosya) |

Kaynak: `apps/lertiva-arc/src-tauri/windows/installer-hooks.nsh`  
Yapılandırma: `tauri.conf.json` → `bundle.windows.nsis.installerHooks`

## Çoklu dosya seçimi (`*`)

| Menü | Komut |
|------|--------|
| Lertiva Arc — seçilenleri sıkıştır | `Lertiva Arc.exe --compress-selection %*` (aynı klasörde **ilk dosya adı**.wlrt — WinRAR gibi) |
| Lertiva Arc — seçilenler şifreli | `Lertiva Arc.exe --compress-selection %* --secure` |
| Lertiva Arc — seçilenler hızlı .wlrt | `Lertiva Arc.exe --compress-selection-quick %*` |

`MultiSelectModel=Document` — WinRAR benzeri çoklu seçim.

Kaldırma: Program kaldırıldığında `NSIS_HOOK_POSTUNINSTALL` anahtarları siler.

Şifreli sıkıştırma (`--secure`): uygulama açılır → **parola sihirbazı** (kurumsal politika varsayılan).

Parolalı RAR/7z için «buraya aç» henüz parola sormaz; uygulama içinden parola girin.

Pilot test listesi: [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md)
