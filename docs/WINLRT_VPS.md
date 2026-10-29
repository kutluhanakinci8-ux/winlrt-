# Turhost VPS — winlrt (bekleme sonrası)

Sunucu panelde **Aktif** + IP geldikten sonra.

## DNS

`winlrt.lerta.com.tr` (veya `get.lerta.com.tr`) → **A** → Turhost IP

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
certbot --nginx -d winlrt.lerta.com.tr
```

## GitHub Actions (ileride)

`main` push → SSH → `git pull && npm run build:web` + nginx reload.
