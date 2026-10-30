# winlrtRAR (Tauri)

WinRAR sınıfı masaüstü kabuk — **Faz 2 beta iskelet**.

- Motor: `crates/winlrt-archive` (ZIP, `.wlrt`; RAR stub)
- Plan: [docs/WINLRTRAR_PLAN.md](../../docs/WINLRTRAR_PLAN.md)
- RAR: [docs/RAR_READING.md](../../docs/RAR_READING.md)

## Geliştirme (Windows / macOS)

Önkoşullar: [Tauri prerequisites](https://tauri.app/start/prerequisites/)

```bash
cd /path/to/winlrt
npm install
npm run dev:rar
```

Linux CI yalnızca `cargo check -p winlrt-rar` (GUI bağımlılıkları gerekmez).

## Build

```bash
npm run build:rar
```

Beta `.exe` dağıtımı Faz 2 kapısı sonrası vitrin indirme sayfasına bağlanır.
