# Explorer sağ tık (Faz 3 beta)

Windows NSIS kurulumu sonrası **SystemFileAssociations** altına ek menüler yazılır (varsayılan «Aç» uygulamasını değiştirmez).

| Menü | Komut |
|------|--------|
| winlrtRAR ile aç | `winlrtRAR.exe "%1"` |
| winlrtRAR — buraya aç | `winlrtRAR.exe --extract-here "%1"` |

## Klasör (Directory)

| Menü | Komut |
|------|--------|
| winlrtRAR ile sıkıştır | `winlrtRAR.exe --compress-folder "%1"` (UI: kaynak + önerilen `.wlrt`) |
| winlrtRAR — şifreli sıkıştır | `winlrtRAR.exe --compress-folder "%1" --secure` (UI: parola + Oluştur) |
| winlrtRAR — hızlı .wlrt | `winlrtRAR.exe --compress-folder-quick "%1"` (Dengeli profil, komşu dosya) |

Kaynak: `apps/winlrt-rar/src-tauri/windows/installer-hooks.nsh`  
Yapılandırma: `tauri.conf.json` → `bundle.windows.nsis.installerHooks`

## Çoklu dosya seçimi (`*`)

| Menü | Komut |
|------|--------|
| winlrtRAR — seçilenleri sıkıştır | `winlrtRAR.exe --compress-selection %*` (aynı klasörde **ilk dosya adı**.wlrt — WinRAR gibi) |
| winlrtRAR — seçilenler şifreli | `winlrtRAR.exe --compress-selection %* --secure` |
| winlrtRAR — seçilenler hızlı .wlrt | `winlrtRAR.exe --compress-selection-quick %*` |

`MultiSelectModel=Document` — WinRAR benzeri çoklu seçim.

Kaldırma: Program kaldırıldığında `NSIS_HOOK_POSTUNINSTALL` anahtarları siler.

Şifreli sıkıştırma (`--secure`): uygulama açılır → **parola sihirbazı** (kurumsal politika varsayılan).

Parolalı RAR/7z için «buraya aç» henüz parola sormaz; uygulama içinden parola girin.

Pilot test listesi: [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md)
