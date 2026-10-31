# G2 — Gerçek SmartScreen / kurulum vitrin PNG

Repo CI imzası (`verify-madde-h-signed.sh`) tamam; vitrin **gerçek ekran** için:

## 1) Windows PC

Imzalı kurulum: Actions → **Lertiva Zip Windows beta build** → artifact `winlrt-zip-windows-bundle`  
(Aynı secret ile **Lertiva Arc**.)

## 2) PNG dosyaları

`apps/web/public/assets/gallery/incoming/`:

| Dosya | İçerik |
|-------|--------|
| `smartscreen-lertiva.png` | SmartScreen veya imza uyarısı (dev: bilinmeyen yayıncı OK) |
| `installer-lertiva-zip-setup.png` | Zip NSIS kurulum sihirbazı |
| `installer-lertiva-arc-setup.png` | Arc NSIS kurulum sihirbazı |

## 3) Vitrin

```bash
bash scripts/import-real-installer-screenshots.sh
npm run build:web
bash scripts/agent-deploy-vitrin.sh
bash scripts/evaluate-lertiva-windows-signing-gate.sh
```

`lertiva-windows-signing-gate.json` → `vitrin_smartscreen_real: true`

Doğrulama: `bash scripts/verify-madde-g2-vitrin.sh`
