# winlrt

winlrt bir **sıkıştırma ve açma** programıdır (Windows, macOS; Android / iOS yol haritasında).

## Repo yapısı

| Yol | Açıklama |
|-----|----------|
| `apps/web` | Tanıtım + indirme vitrin (statik) |
| `apps/desktop` | Masaüstü uygulama (Tauri — yakında) |
| `docs/` | Cursor/GitHub ve Turhost VPS notları |
| `releases/` | Build çıktıları (git’e büyük binary eklemeyin; CI artifact) |

## Yerel geliştirme

```bash
npm install
npm run dev:web    # http://localhost:3020
npm run build:web  # apps/web/dist
```

**winlrtRAR motoru (Rust):**

```bash
cargo build -p winlrt-archive
cargo run -p winlrt-archive -- --help
```

## CI / deploy

- **CI:** GitHub Actions `ci.yml` — vitrin build doğrulaması
- **Canlı site:** [docs/CANLI_SITE.md](docs/CANLI_SITE.md) (DNS, VPS, SSL, GitHub Secrets)
- **VPS notları:** [docs/WINLRT_VPS.md](docs/WINLRT_VPS.md)

## Dokümantasyon

- [Cursor ↔ GitHub](docs/CURSOR_GITHUB.md)
- [Turhost VPS](docs/WINLRT_VPS.md) (sunucu **Aktif** olunca)

## Lisans

TBD
