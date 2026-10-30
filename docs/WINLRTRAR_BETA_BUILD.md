# winlrtRAR beta `.exe` (Faz 2)

## Windows (geliştirici makinesi)

1. [Tauri prerequisites](https://tauri.app/start/prerequisites/) (Windows)
2. Repo kökü:

```bash
npm install
npm run build:rar
```

Çıktı: `apps/winlrt-rar/src-tauri/target/release/bundle/` (MSI/EXE).

## Dağıtım

- İmzalı kurulum Faz 3 kapısı; beta için vitrin **İndir** sayfasına manuel yükleme (Turhost / GitHub Releases).
- Otomatik CI: `.github/workflows/winlrt-rar-windows.yml` (Windows runner).
