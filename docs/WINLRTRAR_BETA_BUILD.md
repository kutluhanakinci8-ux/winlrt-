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

- İmzalı kurulum Faz 3 kapısı.
- **GitHub Actions:** Repo → Actions → **winlrtRAR Windows beta build** → **Run workflow**.
  - Artifact: `winlrt-rar-windows-bundle`
  - Otomatik **prerelease** etiketi: `winlrt-rar-beta.<run_number>` (`.exe` / `.msi`)
- Vitrin: [winlrt.com.tr/indir](https://winlrt.com.tr/indir) → GitHub Releases listesi.
