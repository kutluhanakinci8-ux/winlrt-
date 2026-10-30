# Canlı site — winlrt vitrin (Turhost VPS)

Hedef: **https://winlrt.com.tr** üzerinde statik vitrin (Turhost VPS **37.230.105.99**, SSH port **23422**).

**isimtescil adım adım:** [DNS_ISIMTESCIL.md](./DNS_ISIMTESCIL.md)

## Kontrol listesi

| # | Adım | Kim | Durum |
|---|------|-----|--------|
| 1 | Turhost VPS **Aktif** + public IP | Siz | ☐ |
| 2 | DNS: kök **A** → `37.230.105.99` (+ isteğe `srv`) | isimtescil | ☐ |
| 3 | `main` dalında CI + deploy workflow’ları | GitHub merge | ☐ |
| 4 | VPS bootstrap (`deploy/scripts/vps-bootstrap.sh`) | SSH ile sunucu | ☐ |
| 5 | `certbot --nginx -d winlrt.com.tr -d www.winlrt.com.tr` | SSH `-p 23422` | ☐ |
| 6 | GitHub Actions **Secrets** + deploy anahtarı | GitHub repo ayarları | ☐ |
| 7 | İlk deploy (manuel workflow veya push) | GitHub Actions | ☐ |
| 8 | Tarayıcıda ana sayfa + `/downloads/` test | Siz | ☐ |

---

## 1. DNS

Bkz. **[DNS_ISIMTESCIL.md](./DNS_ISIMTESCIL.md)** — özet: kök `winlrt.com.tr` A kaydını **37.230.105.99** yapın; `srv` yoksa A `srv` → aynı IP.

---

## 2. GitHub — workflow’ların `main`’de olması

Canlı deploy yalnızca **`main`** push’unda (veya manuel tetiklemede) çalışır.

1. Branch **`cursor/github-actions-and-env-ea18`** (ve canlı site ekleri) → **Pull Request** → **Merge**  
   veya yerelde: `git checkout main && git merge cursor/live-site-setup-ea18 && git push origin main`

2. **Actions** sekmesinde **CI** workflow’un yeşil olduğunu doğrulayın.

---

## 3. VPS — ilk kurulum

Sunucuya SSH (Turhost panelindeki root / ubuntu kullanıcısı):

```bash
# Örnek: repoyu geçici indirip script çalıştırma
git clone https://github.com/kutluhanakinci8-ux/winlrt-.git /tmp/winlrt-setup
cd /tmp/winlrt-setup
chmod +x deploy/scripts/*.sh

export WINLRT_DOMAIN=winlrt.com.tr
# İsteğe bağlı: deploy kullanıcısı ile clone için deploy key ekleyin
bash deploy/scripts/vps-bootstrap.sh
```

Script: paketler, `git clone` → `/var/www/winlrt`, `npm ci && npm run build:web`, nginx site config.

**SSH ile private clone** için GitHub’da sunucuya **Deploy key** (read-only) ekleyin; clone URL:  
`git@github.com:kutluhanakinci8-ux/winlrt-.git`

---

## 4. SSL (HTTPS)

DNS sunucuya işaret ettikten sonra:

```bash
certbot --nginx -d winlrt.com.tr -d www.winlrt.com.tr
```

Yenileme: certbot timer (paket kurulumunda gelir).

---

## 5. GitHub Actions — deploy anahtarı ve secrets

### 5a. Deploy kullanıcısı (önerilen)

VPS’te:

```bash
adduser --disabled-password --gecos "" winlrtdeploy
mkdir -p /var/www/winlrt/apps/web/dist
chown -R winlrtdeploy:www-data /var/www/winlrt/apps/web/dist
chmod -R g+w /var/www/winlrt/apps/web/dist
```

### 5b. Sadece deploy için SSH anahtarı

**Kendi bilgisayarınızda** (private key GitHub’a gidecek, sunucuya asla commit etmeyin):

```bash
ssh-keygen -t ed25519 -f winlrt-gha-deploy -N ""
cat winlrt-gha-deploy.pub
```

Public key’i VPS’te `winlrtdeploy` kullanıcısına:

```bash
sudo -u winlrtdeploy mkdir -p ~winlrtdeploy/.ssh
sudo -u winlrtdeploy bash -c 'cat >> ~winlrtdeploy/.ssh/authorized_keys'  # pubkey yapıştır
sudo chmod 700 ~winlrtdeploy/.ssh
sudo chmod 600 ~winlrtdeploy/.ssh/authorized_keys
```

Private key dosyasının **tam içeriği** → GitHub **Settings → Secrets → Actions → New repository secret**

| Secret | Değer |
|--------|--------|
| `VPS_HOST` | `37.230.105.99` veya `srv.winlrt.com.tr` |
| `VPS_PORT` | `23422` |
| `VPS_USER` | `winlrtdeploy` |
| `VPS_SSH_KEY` | `winlrt-gha-deploy` private key (PEM) |
| `VPS_WEB_ROOT` | `/var/www/winlrt/apps/web/dist` |
| `VPS_PORT` | (boş bırakılabilir; varsayılan 22) |

### 5c. nginx reload (isteğe bağlı)

Deploy workflow nginx’i yeniden yükler. `winlrtdeploy` için:

```bash
echo 'winlrtdeploy ALL=(ALL) NOPASSWD: /usr/sbin/nginx -t, /bin/systemctl reload nginx' \
  | sudo tee /etc/sudoers.d/winlrtdeploy-nginx
sudo chmod 440 /etc/sudoers.d/winlrtdeploy-nginx
```

Reload adımını atlamak isterseniz workflow’daki “Reload nginx” adımını kaldırabilirsiniz; statik dosya güncellemesi reload gerektirmez.

### 5d. Otomatik deploy

**Settings → Secrets and variables → Actions → Variables**

- `VPS_DEPLOY_ENABLED` = `true` → `main`’e vitrin dosyası push’unda deploy  
- `false` / yok → yalnızca **Actions → Deploy vitrin to VPS → Run workflow**

---

## 6. İlk deploy testi

1. GitHub → **Actions** → **Deploy vitrin to VPS** → **Run workflow** (branch: `main`)
2. Log’da SCP ve (varsa) nginx reload başarılı mı bakın
3. https://winlrt.com.tr açın

---

## 7. Sorun giderme

| Belirti | Olası neden |
|---------|-------------|
| 502 / connection refused | nginx çalışmıyor: `systemctl status nginx` |
| 404 | `VPS_WEB_ROOT` yanlış veya dist boş; yerelde `npm run build:web` |
| SCP permission denied | `winlrtdeploy` hedef dizine yazamıyor; `chown` / `chmod` |
| certbot failed | DNS henüz IP’ye gelmemiş |
| Workflow hiç çalışmıyor | Workflow’lar `main`’de değil veya `VPS_DEPLOY_ENABLED` kapalı |

Detay: [WINLRT_VPS.md](./WINLRT_VPS.md)
