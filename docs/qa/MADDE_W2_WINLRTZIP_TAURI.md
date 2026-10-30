# Madde W2 — Lertiva Zip Tauri kabuk (MVP) %100

## Kapsam

| Bileşen | Konum |
|---------|--------|
| Tauri 2 + React | `apps/lertiva-zip/` |
| Komutlar | `list_zip`, `extract_zip`, `create_zip`, `test_zip`, `engine_info` |
| Motor | `zip_io` (ortak crate) |
| CI | `cargo check -p lertiva-zip` |

## Geliştirme

```bash
npm ci
npm run dev:zip    # Tauri + Vite :1422
npm run build:zip  # Windows NSIS (yerel)
```

Doğrulama: `bash scripts/verify-madde-w2.sh`

Windows workflow / imzalı installer: Lertiva Arc beta hattına benzer şekilde sonraki madde (W3).
