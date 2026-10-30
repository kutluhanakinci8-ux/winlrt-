# Madde W3 — winlrtZIP Windows CI + vitrin indirme %100

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Workflow | `.github/workflows/winlrt-zip-windows.yml` |
| Build tetik | `.github/flags/trigger-zip-beta-build` |
| Release tetik | `.github/flags/trigger-zip-beta-release` |
| Agent | `scripts/agent-madde-w3-complete.sh` |
| Vitrin | `/downloads/` winlrtZIP beta linki |

## W3 %100

1. Son `winlrt-zip-beta.*` pre-release + NSIS `*-setup.exe`
2. Artifact adı `winlrt-zip-windows-bundle`
3. `bash scripts/verify-madde-w3.sh`

Imza: `TAURI_SIGNING_*` yoksa imzasız (winlrtRAR ile aynı politika).

**Doğrulandı:** [winlrt-zip-beta.1](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-zip-beta.1) · run `41097691073` · `winlrtZIP_0.1.0_x64-setup.exe`
