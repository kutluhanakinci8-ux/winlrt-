# Explorer sağ tık (Faz 3 beta)

Windows NSIS kurulumu sonrası **SystemFileAssociations** altına ek menüler yazılır (varsayılan «Aç» uygulamasını değiştirmez).

| Menü | Komut |
|------|--------|
| winlrtRAR ile aç | `winlrtRAR.exe "%1"` |
| winlrtRAR — buraya aç | `winlrtRAR.exe --extract-here "%1"` |

Kaynak: `apps/winlrt-rar/src-tauri/windows/installer-hooks.nsh`  
Yapılandırma: `tauri.conf.json` → `bundle.windows.nsis.installerHooks`

Kaldırma: Program kaldırıldığında `NSIS_HOOK_POSTUNINSTALL` anahtarları siler.

Parolalı RAR/7z için Explorer menüsünden açma henüz parola sormaz; uygulama içinden parola girin.
