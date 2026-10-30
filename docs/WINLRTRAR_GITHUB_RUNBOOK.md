# GitHub Actions — winlrt runbook

Repo: `kutluhanakinci8-ux/winlrt-`

| Workflow | Tetikleyici | Amaç |
|----------|-------------|------|
| **Main pipeline** | `main` push, PR, manual | Vitrin build, rust test, bench, VPS deploy (secret varsa) |
| **winlrt Windows benchmark (resmi)** | `main` (bench paths), manual | Resmi KPI + 7-Zip/WinRAR; artifact `bench-report-windows` |
| **winlrtRAR Windows beta build** | manual + beta flags | Tauri beta + Release; imza verify (secret varsa) |
| **winlrtZIP Windows beta build** | `trigger-zip-beta-*` push | winlrtZIP NSIS beta + Release |
| **Vitrin KPI samples (Linux)** | manual | Linux KPI JSON → artifact `vitrin-kpi-samples` |
| **Deploy VPS** | (varsa ayrı) | Ek deploy — bkz. `deploy-vps.yml` |

## Merge sonrası sıra

Bkz. [WINLRTRAR_POST_MERGE.md](./WINLRTRAR_POST_MERGE.md).

## Manuel çalıştırma (`gh`)

```bash
gh auth login
gh workflow run "Main pipeline" --repo kutluhanakinci8-ux/winlrt- -r main
gh workflow run "winlrt Windows benchmark (resmi)" --repo kutluhanakinci8-ux/winlrt- -r main
gh workflow run "winlrtRAR Windows beta build" --repo kutluhanakinci8-ux/winlrt- -r main
gh workflow run "Vitrin KPI samples (Linux)" --repo kutluhanakinci8-ux/winlrt- -r main
```

Windows benchmark strict:

```bash
gh workflow run "winlrt Windows benchmark (resmi)" --repo kutluhanakinci8-ux/winlrt- -r main \
  -f kpi_strict=true
```

Beta build (Release atlamadan):

```bash
gh workflow run "winlrtRAR Windows beta build" --repo kutluhanakinci8-ux/winlrt- -r main \
  -f skip_release=true
```

## Secrets / variables

| Ad | Amaç |
|----|------|
| `VPS_HOST`, `VPS_PORT`, `VPS_USER`, `VPS_SSH_KEY` / `B64` / `PASSWORD`, `VPS_WEB_ROOT` | Deploy |
| `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | İmza |
| Variable `WINLRT_KPI_STRICT=1` | KPI kapısı fail (Linux + Windows bench) |

Mac secret yükleme: `scripts/mac-set-github-vps-secret.sh`, `scripts/mac-set-github-tauri-signing-secrets.sh`

## CI izleme (Cloud Agent)

Merge push sonrası agent, dal CI’sını `cursor-subscriptions` **subscribe_github_ci** ile bekleyebilir; siz merge etmeden önce agent koşuları yalnızca feature dal push’larında tetiklenir (çoğu workflow `main` odaklı).
