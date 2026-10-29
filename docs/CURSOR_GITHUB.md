# Cursor ↔ GitHub (winlrt)

## 1. GitHub hesabını Cursor’a bağla

1. Cursor → **Settings** (Ctrl+,) → **Account** / **General**
2. **Sign in to GitHub** → tarayıcıda yetkilendir
3. Gerekirse: **Settings → Git** → kullanıcı adı / e-posta

Alternatif terminal:

```bash
gh auth login
```

## 2. Repoyu bilgisayarda aç

```bash
git clone https://github.com/kutluhanakinci8-ux/winlrt-.git
cd winlrt-
cursor .
```

Windows’ta `winlrt-` klasörünü Cursor ile **File → Open Folder**.

## 3. Değişiklik gönderme

Cursor sol çubuk **Source Control** (Ctrl+Shift+G):

1. Değişiklikleri stage (+)
2. Commit mesajı
3. **Sync / Push**

veya:

```bash
git add -A
git commit -m "feat: ..."
git push origin main
```

## 4. Turhost VPS (Aktif olunca)

Aynı repo sunucuda:

```bash
git clone git@github.com:kutluhanakinci8-ux/winlrt-.git /var/www/winlrt
```

Deploy key: GitHub repo → Settings → Deploy keys → sunucu `~/.ssh/id_ed25519.pub`

Vitrin: `apps/web/dist` → nginx root (bkz. `docs/WINLRT_VPS.md` — VPS aktif olunca doldurulacak).
