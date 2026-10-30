# libunrar gömülü backend (`unrar-embedded`)

Windows beta `.exe` derlemesi `unrar-embedded` feature ile **libunrar** (unrar-ng) statik link eder — RAR4/RAR5 uyumluluğu WinRAR/UnRAR CLI olmadan.

## Derleme

```bash
cd apps/winlrt-rar
npm run tauri build -- --features unrar-embedded
```

GitHub Actions: `.github/workflows/winlrt-rar-windows.yml`

Linux geliştirici CI (`cargo check -p winlrt-rar`) feature **kapalı**; RAR5 fixture `rar` crate + isteğe bağlı sistem `unrar`.

## Yan dosya (opsiyonel)

`WINLRT_UNRAR` ortam değişkeni veya `winlrtRAR.exe` yanına `unrar.exe` — CLI backend önce yan dosyayı dener.

## Lisans

Dağıtım öncesi [UnRAR kaynak lisansı](https://www.rarlab.com/rar_add.htm) hukuk onayı gerekir (bkz. RAR_READING.md).
