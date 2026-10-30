# Madde W14 — Lertiva Zip macOS build (Faz 3) %100

[WINLRTZIP_PLAN.md](../WINLRTZIP_PLAN.md) Faz 3: **macOS build** (notarize secret sonrası).

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Tauri macOS bundle | `tauri.conf.json` → `bundle.macOS`, `macos/Entitlements.plist` |
| CI | `.github/workflows/lertiva-zip-macos.yml` |
| Tetikleyici | `.github/flags/trigger-zip-macos-build` |
| Notarize rehberi | [WINLRTZIP_MACOS_NOTARIZE.md](./WINLRTZIP_MACOS_NOTARIZE.md) |
| Platform metni | `engine_info` macOS ipucu |

## W14 %100 (repo kapısı)

1. `bash scripts/verify-madde-w14.sh`
2. İlk yeşil koşu: Actions → **Lertiva Zip macOS beta build** (artifact `winlrt-zip-macos-bundle`)
3. Notarization: `APPLE_*` secret — kapı ayrı; imzasız dmg beta kabul

## Yerel (Mac)

```bash
npm ci
npm run build:zip -- --bundles dmg
```
