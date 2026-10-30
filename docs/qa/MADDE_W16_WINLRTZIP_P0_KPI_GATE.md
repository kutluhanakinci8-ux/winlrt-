# Madde W16 — Lertiva Zip P0×3 KPI kapısı %100

Faz 3 kapı: **hız + sıkıştırma + kurumsal güven** (Lertiva Zip ZIP hattı).

| P0 | Ölçüm | Kaynak |
|----|--------|--------|
| Sıkıştırma | `zip_kpi.compression_gate` | Balanced `--zip-kpi` |
| Hız | `zip_kpi.speed_gate` | Fast `--zip-kpi` |
| Güven | AES-256 I4 + `zip_io` | `cargo test i4_aes256_roundtrip` |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Değerlendirme | `scripts/evaluate-winlrtzip-p0-gate.sh` |
| Windows → vitrin | `scripts/import-windows-zip-kpi-to-vitrin.sh` |
| Yerel yenileme | `scripts/refresh-winlrtzip-p0-gate.sh` |
| Özet JSON | `apps/web/public/benchmark/samples/winlrtzip-p0-gate.json` |
| Windows CI | `evaluate-bench-zip-kpi.ps1` + strict (isteğe) |

Strict: `WINLRT_ZIP_KPI_STRICT=1` veya `WINLRT_KPI_STRICT=1`.

macOS resmi satır: W14 dmg CI / Mac agent bench → vitrin import (placeholder: `winlrtzip-macos-*`).

## W16 %100

1. `bash scripts/verify-madde-w16.sh`
2. Main pipeline: P0 gate adımı (Linux proxy)
3. Resmi Windows: artifact → `import-windows-zip-kpi-to-vitrin.sh` → deploy
