# Lertiva

**Lertiva Arc** (arşiv / `.wlrt`) ve **Lertiva Zip** (*Lertiva Pack* alternatif ad) — yerli sıkıştırma platformu (Windows, macOS; mobil yol haritasında).

## Repo yapısı

| Yol | Açıklama |
|-----|----------|
| `apps/web` | Tanıtım + indirme vitrin (statik) |
| `apps/lertiva-arc` | Lertiva Arc Tauri kabuk |
| `apps/lertiva-zip` | Lertiva Zip Tauri kabuk |
| `crates/lertiva-archive` | Ortak arşiv motoru |
| `docs/` | Plan, QA, deploy |
| `releases/` | Build çıktıları (CI artifact) |

## Yerel geliştirme

```bash
npm install
npm run dev:web    # http://localhost:3020
npm run build:web  # apps/web/dist
npm run dev:arc    # Lertiva Arc Tauri
npm run dev:zip    # Lertiva Zip Tauri
```

**Motor (Rust):**

```bash
cargo build -p lertiva-archive
cargo run -p lertiva-archive --bin lertiva-zip -- --help
```

## Marka doğrulama

```bash
bash scripts/verify-lertiva-brand.sh
```

## CI / deploy

- **Otomatik:** `main` push → [Main pipeline](.github/workflows/main-pipeline.yml)
- **Canlı vitrin:** https://winlrt.com.tr (alan adı geçiş sürecinde)

## Dokümantasyon

- [Suite yol haritası](docs/SUITE_ROADMAP.md) · [Lertiva Arc plan](docs/WINLRTRAR_PLAN.md) · [Lertiva Zip plan](docs/WINLRTZIP_PLAN.md)
