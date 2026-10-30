# Madde E + F — GitHub VPS + deploy %100

## Neden eksikti?

| Madde | Sorun |
|-------|--------|
| **E** | `VPS_HOST` var; `VPS_SSH_KEY` bozuk; `VPS_SSH_PASSWORD` yok → CI `auth=skip` |
| **F** | Agent rsync OK; **Actions deploy** rsync adımı **skipped** |
| **Agent** | `gh secret set` → **403** (integration token) |

## %100 tanımı (bu proje)

1. **Agent rsync:** `post-phase-deploy.sh` (her push sonrası agent).
2. **VPS otomatik:** `/usr/local/bin/winlrt-vitrin-sync.sh` + cron **10 dk** (`origin/main` → build → dist).
3. Doğrulama: `bash scripts/verify-madde-ef.sh`

GitHub Secrets panelden `VPS_SSH_PASSWORD` eklenirse CI deploy da yeşil olur (ekstra).

## Kurulum (agent)

```bash
VPS_SSH_PASSWORD='…' bash scripts/agent-madde-ef-vps-cron.sh
```
