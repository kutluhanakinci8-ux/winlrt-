# Madde W15 — WinZip geçiş + macOS KPI vitrin %100

Faz 3: [WINLRTZIP_PLAN.md](../WINLRTZIP_PLAN.md) — geçiş sihirbazı + KPI vitrin satırı.

## WinZip geçiş sihirbazı

| Bileşen | Konum |
|---------|--------|
| Mantık | `apps/winlrt-zip/src-tauri/src/migration.rs` |
| Tauri | `winzip_migration_guide` |
| UI | `App.tsx` — **WinZip geçiş** modal |
| Rehber | [WINLRTZIP_WINZIP_MIGRATION.md](./WINLRTZIP_WINZIP_MIGRATION.md) |

## macOS KPI vitrin

| Bileşen | Konum |
|---------|--------|
| Örnek JSON | `apps/web/public/benchmark/samples/winlrtzip-macos-*-kpi.json` |
| Yenileme | `bash scripts/refresh-winlrtzip-vitrin-samples.sh` |
| Vitrin | `/benchmark/` kartları |

Placeholder: Linux motor proxy; resmi macOS satırı dmg CI / Mac agent ile güncellenir.

## W15 %100

1. `bash scripts/verify-madde-w15.sh`
2. Main pipeline: migration test + vitrin dosyaları
