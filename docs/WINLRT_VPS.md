# Turhost VPS — winlrt (bekleme sonrası)

Sunucu panelde **Aktif** + IP geldikten sonra.

**Adım adım canlı site:** [CANLI_SITE.md](./CANLI_SITE.md)

## DNS

`winlrt.com.tr` → **A** → `37.230.105.99` (Turhost VPS) · isteğe bağlı `srv` → aynı IP

## İlk kurulum (Ubuntu)

```bash
apt update && apt upgrade -y
apt install -y git curl nginx certbot python3-certbot-nginx ufw
ufw allow OpenSSH && ufw allow 'Nginx Full' && ufw --force enable
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt install -y nodejs
```

## Uygulama

```bash
git clone git@github.com:kutluhanakinci8-ux/winlrt-.git /var/www/winlrt
cd /var/www/winlrt && npm install && npm run build:web
```

nginx `root` → `/var/www/winlrt/apps/web/dist`

```bash
certbot --nginx -d winlrt.com.tr -d www.winlrt.com.tr
```

## GitHub Actions

| Workflow | Açıklama |
|----------|----------|
| `.github/workflows/main-pipeline.yml` | `main` push: CI + otomatik deploy (secrets varsa) + VPS yedek |

### Repo secrets (Settings → Secrets and variables → Actions)

| Secret | Örnek |
|--------|--------|
| `VPS_HOST` | Turhost IP veya FQDN |
| `VPS_USER` | `root` veya deploy kullanıcısı |
| `VPS_SSH_KEY` | Özel anahtar (PEM), sunucuda public key yetkili |
| `VPS_WEB_ROOT` | `/var/www/winlrt/apps/web/dist` |
| `VPS_PORT` | (isteğe bağlı) SSH portu, varsayılan 22 |

**main** dalına her push otomatik deploy dener (secrets tanımlıysa). Ayrıntı: [AUTO_DEPLOY.md](./AUTO_DEPLOY.md).

### Cloud Agent ortamı

Kökte `environment.json` → `npm install` build anında çalışır; Cursor ortam build’i sonrası agent’lar daha hızlı açılır.
