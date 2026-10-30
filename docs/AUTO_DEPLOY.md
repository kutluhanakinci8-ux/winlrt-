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
| `VPS_SSH_KEY` | OpenSSH private key (`~/.ssh/winlrt-gha-deploy`) — **veya** aşağıdaki Base64 |
| `VPS_SSH_KEY_B64` | Aynı anahtarın **tek satır** Base64 hali |
| `VPS_SSH_PASSWORD` | root SSH **şifresi** (anahtar GitHub’a giremiyorsanız; Mac’teki `ssh -i` ile karıştırmayın) |
| `VPS_WEB_ROOT` | `/var/www/winlrt/apps/web/dist` |

Kimlik doğrulama: **`VPS_SSH_KEY` veya `VPS_SSH_KEY_B64` veya `VPS_SSH_PASSWORD`** (en az biri).

Mac’te `ssh -i ~/.ssh/winlrt-gha-deploy ... echo OK` **yalnızca** anahtarın VPS’te kayıtlı olduğunu gösterir. GitHub Actions ayrı bir ortamdır; aynı anahtar veya şifre **repo Secrets**’a kaydedilmeden deploy çalışmaz.

### Mac’te secret kutusuna yapıştırma çalışmıyorsa

**Önerilen (yapıştırma yok):** repoyu Mac’e clone etmişseniz veya sadece script’i indirip:

```bash
bash scripts/mac-set-github-vps-secret.sh
```

Script gerekirse `brew install gh` yapar; `gh auth login` tarayıcı ile oturum açar; anahtarı **dosyadan** `VPS_SSH_KEY` olarak yükler.

**Alternatif (tek satır secret):**

```bash
bash scripts/mac-vps-key-base64-for-github.sh
```

Masaüstündeki `VPS_SSH_KEY_B64.txt` içeriğini GitHub’da secret adı **`VPS_SSH_KEY_B64`** olarak kaydedin (tek satır).

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
