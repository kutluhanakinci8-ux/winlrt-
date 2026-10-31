# Madde W34 — Lertiva Zip macOS resmi zip_kpi (G1/G3)

**Amaç:** Vitrin `lertiva-zip-macos-*-kpi.json` → `source: macos-official-ci` (placeholder kalkar).

## CI

- Workflow: `lertiva-zip-macos.yml` → job `macos-zip-kpi` (7-Zip + `bench --zip-kpi`)
- Artifact: `lertiva-zip-macos-bench`
- Import: `bash scripts/import-macos-zip-kpi-to-vitrin.sh <balanced.json> <fast.json>`

## Doğrulama

```bash
bash scripts/verify-madde-w34.sh
```

## Secret (G3)

`APPLE_*` — dmg imza/notarize (W14); KPI bench secret gerektirmez.
