# Madde W6 — Lertiva Zip benchmark (7-Zip / WinZip satırı) %100

## Kapsam

| Bileşen | Konum |
|---------|--------|
| Rakip satırları | `bench.rs` — `seven_zip_*`, `winzip_*`, `winrar_*` (ZIP) |
| Lertiva Zip KPI | `evaluate_zip_product_kpi` → JSON `zip_kpi` |
| CLI | `cargo run -p lertiva-archive -- bench --all --json --zip-kpi` |
| Windows CI | `.github/workflows/bench-windows.yml` + `evaluate-bench-zip-kpi.ps1` |
| Vitrin | `apps/web/public/benchmark/samples/lertiva-zip-*.json` |

## Ortam

| Değişken | Anlam |
|----------|--------|
| `WINLRT_BENCH_INCLUDE_7Z=1` | 7-Zip satırı |
| `WINLRT_BENCH_INCLUDE_WINRAR=1` | WinRAR ZIP modu |
| `WINLRT_BENCH_INCLUDE_WINZIP=1` | WinZip WZZIP/WZUNZIP (kuruluysa) |

WinZip satırı agent’ta yoksa KPI yalnızca 7-Zip/WinRAR ile hesaplanır.

## W6 %100

1. `bash scripts/verify-madde-w6.sh`
2. Windows resmi bench artifact: `vitrin-lertiva-zip-balanced-kpi.json`, `vitrin-lertiva-zip-fast-kpi.json`
3. Yerel örnek: `bash scripts/refresh-lertiva-zip-vitrin-samples.sh`
