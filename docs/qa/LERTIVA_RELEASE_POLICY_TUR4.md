# Tur 4 — Lertiva release politikası (G10)

**Amaç:** Kullanıcıya dönük isimler `Lertiva *`; altyapıda bilinçli dual (`winlrt.com.tr`, `lertiva-*` paketleri, eski `winlrt-*` git tag geçmişi).

## Kurallar

| Alan | Politika |
|------|----------|
| Ürün adı | Lertiva Arc · Lertiva Zip (vitrin, installer, mağaza metni) |
| Domain | `winlrt.com.tr` vitrin (değişmez); `lerta.com.tr` ayrı ürün |
| Rust crate / npm | `lertiva-*` (`lertiva-archive`, `apps/lertiva-arc`, …) |
| GitHub workflow | `lertiva-*-windows.yml` / `lertiva-zip-macos.yml` (eski `winlrt-*` yok) |
| Release tag | Yeni: `lertiva-zip-v*` / `lertiva-arc-v*` tercih; geçmiş `winlrt-*` tag’leri arşiv |
| Repo taraması | `scripts/verify-lertiva-brand.sh` (CI main-pipeline) |

## Suite vizyon (Tur 4 vitrin)

- Adım 1–2: Arc + Zip (aktif)
- Adım 3: PDF / imza / sertifika — [PRODUCT_ROADMAP.md](../PRODUCT_ROADMAP.md) madde 4 kickoff (placeholder vitrin metni)
- Adım 4: Lerta üretim — G8

Doğrulama: `bash scripts/verify-faz-g10.sh`
