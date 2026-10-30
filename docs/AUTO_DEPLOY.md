# Otomatik merge + deploy

## Akış

1. Geliştirme **`main`** dalına merge + push (Cloud Agent bunu yapar).
2. GitHub Actions **`Main pipeline`** çalışır (`main` push veya **Run workflow**):
   - `npm ci` + vitrin build
   - `cargo build` (winlrt-archive)
   - **main push** → VPS’te **yedek** + **dist deploy** + nginx reload

Workflow: `.github/workflows/main-pipeline.yml`

## GitHub Secrets (bir kez)

Repo → **Settings → Secrets and variables → Actions**

| Secret | winlrt.com.tr örneği |
|--------|----------------------|
| `VPS_HOST` | `37.230.105.99` |
| `VPS_PORT` | `23422` |
| `VPS_USER` | `root` |
| `VPS_SSH_KEY` | Private key (PEM) |
| `VPS_WEB_ROOT` | `/var/www/winlrt/apps/web/dist` |

Secrets tanımlı değilse deploy **atlanır** (uyarı log); CI yine çalışır.

Manuel **Run workflow** her zaman **güncel `main`** commit’inde çalışır; `deploy` job’u gri (0s) ise eski workflow commit’i kullanılmış olabilir — sayfayı yenileyip tekrar deneyin.

`VPS_DEPLOY_ENABLED` artık **gerekmez** — main push yeterli.

## VPS yedekleri

Her deploy öncesi:

```text
/var/backups/winlrt/web-dist/<UTC-timestamp>-<git-sha>/
```

Son **5** yedek tutulur; eskiler silinir.

## Yerel (acil)

```bash
cd /var/www/winlrt && git pull && npm ci && npm run build:web
```

## Repo arşivi

Eski dosyalar: [archive/README.md](../archive/README.md)  
Anlık snapshot: `bash scripts/repo-archive-snapshot.sh`
