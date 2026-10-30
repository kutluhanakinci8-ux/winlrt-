# Madde W2 — winlrtZIP Tauri kabuk (MVP) %100

## Kapsam

| Bileşen | Konum |
|---------|--------|
| Tauri 2 + React | `apps/winlrt-zip/` |
| Komutlar | `list_zip`, `extract_zip`, `create_zip`, `test_zip`, `engine_info` |
| Motor | `zip_io` (ortak crate) |
| CI | `cargo check -p winlrt-zip` |

## Geliştirme

```bash
npm ci
npm run dev:zip    # Tauri + Vite :1422
npm run build:zip  # Windows NSIS (yerel)
```

Doğrulama: `bash scripts/verify-madde-w2.sh`

Windows workflow / imzalı installer: winlrtRAR beta hattına benzer şekilde sonraki madde (W3).
