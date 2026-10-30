# Dalga 1 — GitHub VPS secret (isteğe bağlı: CI deploy)

**Canlı deploy (v1.0 sonrası):** Agent `post-phase-deploy.sh` — **birincil yol**; GitHub secret zorunlu değil.

**Canlı deploy:** Cloud Agent zaten yetkili `VPS_*` ile `post-phase-deploy.sh` çalıştırır — şifre repoya yazılmaz.

**CI (push → Actions deploy):** Agent token **403** ile GitHub Secret yazamaz. İsterseniz **bir kez** kendi GitHub oturumunuzdan:

1. https://github.com/kutluhanakinci8-ux/winlrt-/settings/secrets/actions  
2. **New repository secret**  
   - `VPS_HOST` = `37.230.105.99`  
   - `VPS_PORT` = `23422`  
   - `VPS_USER` = `root`  
   - `VPS_WEB_ROOT` = `/var/www/winlrt/apps/web/dist`  
   - **`VPS_SSH_PASSWORD`** = root SSH şifresi (agent deploy ile aynı)  
3. Bozuk varsa silin: `VPS_SSH_KEY`, `VPS_SSH_KEY_B64`  
4. Actions → **Main pipeline** → Run workflow (`main`)

**Başarı:** deploy job log’da `auth=password`, `auth=skip` **değil**.

Mac: `bash scripts/mac-set-github-vps-secret.sh` (anahtar tercih ederseniz).

Agent deploy (secret olmadan): `bash scripts/post-phase-deploy.sh` (ortam `VPS_*`).
