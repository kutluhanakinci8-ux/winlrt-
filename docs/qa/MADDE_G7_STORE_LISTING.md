# Faz G7 — Mağaza listing (repo kapısı)

| Kapı | Doğrulama |
|------|-----------|
| W27 mağaza ön koşul | `verify-madde-w27.sh` |
| W32 pilot hazırlık | `verify-madde-w32.sh` |
| Data safety şablon | `WINLRTZIP_STORE_DATA_SAFETY.md` |
| Listing URL (opsiyonel) | `apps/web/public/benchmark/samples/lertiva-store-listing-url.txt` |

```bash
bash scripts/evaluate-lertiva-store-listing-gate.sh
bash scripts/verify-faz-g7.sh
```

**Strateji (onaylı):** Önce Windows/macOS masaüstü → sonra **yalnızca Google Play internal**; App Store sonra. Bkz. [G7_DESKTOP_FIRST_PLAY_INTERNAL.md](./G7_DESKTOP_FIRST_PLAY_INTERNAL.md).

**Ürün:** Play Console internal testing URL → `lertiva-store-listing-url.txt` (App Store bu fazda **yok**).

Şablon: `lertiva-store-listing-url.txt.example`
