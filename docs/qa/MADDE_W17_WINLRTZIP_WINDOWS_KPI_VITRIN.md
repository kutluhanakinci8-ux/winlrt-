# Madde W17 — Lertiva Zip resmi Windows KPI vitrin import %100

[W16](./MADDE_W16_WINLRTZIP_P0_KPI_GATE.md) P0×3 özetinin **Windows resmi** kaynaktan vitrine yazılması.

## Akış

1. Actions → **winlrt Windows benchmark (resmi)** → yeşil koşu
2. Artifact `bench-report-windows` (veya agent tetikleyici)
3. `bash scripts/import-windows-zip-kpi-from-run.sh [run-id]`
4. Vitrin: `winlrtzip-windows-{balanced,fast}-kpi.json`, `winlrtzip-p0-gate.json`
5. Deploy → [/benchmark/](https://winlrt.com.tr/benchmark/)

Otomasyon: `bash scripts/agent-madde-w17-windows-zip-kpi-import.sh`

## W17 %100

1. `bash scripts/verify-madde-w17.sh`
2. Vitrin dosyalarında `source: windows-official-ci` (import sonrası)
3. `/benchmark/` kartları Windows satırını gösterir
