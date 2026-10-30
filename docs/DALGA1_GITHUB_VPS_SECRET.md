# Dalga 1 — GitHub VPS secret (CI otomatik deploy)

Agent token **403** ile secret yazamaz; **bir kez sizin** GitHub hesabınızdan:

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
