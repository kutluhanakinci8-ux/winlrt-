# Cloud Agent — yerel kimlik bilgileri

Agent **deploy ve vitrin basic auth** için repo kökündeki **`.env.vps.local`** dosyasını otomatik yükler (`scripts/lib/load-vps-deploy-env.sh`). Bu dosya **repoya gitmez**.

## Doldurulacak alanlar

| Değişken | Kullanım |
|----------|----------|
| `VPS_SSH_PASSWORD` veya `VPS_SSH_KEY_*` | VPS rsync / nginx sync |
| `VITRIN_BASIC_AUTH_USER` / `VITRIN_BASIC_AUTH_PASSWORD` | Canlı site `curl -u` ve nginx htpasswd |
| `LERTIVA_DEV_CODESIGN_PASSWORD` | (Opsiyonel) `.github/signing/lertiva-dev-codesign.pfx.enc` decrypt — **EV/OV değil** |

Şablon: `docs/env-vps-local.example` → `cp … .env.vps.local`

## GitHub Actions secret’ları

Cloud Agent token’ı çoğu repoda **`gh secret set`** ve **`workflow_dispatch`** için **403** verir. İmza (AP-1) için:

1. Siz: Settings → Secrets → `LERTIVA_DEV_CODESIGN_PASSWORD` (+ gerekirse `GITHUB_ADMIN_PAT`)
2. Actions → **Provision TAURI signing secrets (dev)** → Run
3. Agent: `bash scripts/agent-arc-ap1-publish.sh v1.0.2` (tag; imzalı stable release)

Agent vitrin/VPS işlerini **`.env.vps.local` ile tekrar sormadan** yapmalıdır.
