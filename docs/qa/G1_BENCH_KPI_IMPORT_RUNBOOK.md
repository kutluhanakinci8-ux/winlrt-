# G1 — Bench KPI vitrin import (Windows + macOS)

Resmi P0×3 ve vitrin JSON güncelleme — **CI artifact → `apps/web/public/benchmark/samples/`**.

## Windows (Lertiva Zip P0)

```bash
# En son başarılı bench-windows run
bash scripts/import-windows-zip-kpi-from-run.sh

# Belirli run id
bash scripts/import-windows-zip-kpi-from-run.sh 41177355144

# Elde JSON varsa
bash scripts/import-windows-zip-kpi-to-vitrin.sh bench-report-windows.json bench-report-windows-fast.json
```

Doğrulama: `jq .all_pass apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json`

## macOS (W34 zip_kpi)

```bash
bash scripts/import-macos-zip-kpi-from-run.sh
bash scripts/import-macos-zip-kpi-to-vitrin.sh macos-bench-balanced.json macos-bench-fast.json
```

Doğrulama: `bash scripts/verify-madde-w34.sh`

## Deploy

```bash
bash scripts/evaluate-lertiva-g1-platform-gate.sh
bash scripts/gap-phase-finish.sh 1 --commit "chore: KPI vitrin import"
```

Workflow: `bench-windows.yml` · `lertiva-zip-macos.yml` (artifact `lertiva-zip-macos-bench`).
