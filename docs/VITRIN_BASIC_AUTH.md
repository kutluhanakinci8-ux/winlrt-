# Canlı vitrin erken erişim şifresi

Dış ziyaretçileri geçici olarak engellemek için **yalnızca VPS nginx** katmanında HTTP Basic Auth kullanılır.

## Geliştirmeyi etkilemez

| Ortam | Auth |
|--------|------|
| `npm run build:web` / `apps/web/public` yerel | Hayır |
| GitHub Releases / indirme asset’leri | Hayır (farklı host) |
| `https://winlrt.com.tr` | Evet (şifre `.env.vps.local` içinde) |

## Açma / güncelleme

```bash
cp docs/env-vps-local.example .env.vps.local
# VITRIN_BASIC_AUTH_PASSWORD=... doldurun
bash scripts/vps-sync-vitrin-basic-auth.sh
```

Deploy sonrası otomatik: `VITRIN_BASIC_AUTH_PASSWORD` doluysa `agent-deploy-vitrin.sh` auth’u senkronlar.

## Kapatma (herkese açık vitrin)

```bash
VITRIN_BASIC_AUTH_DISABLE=1 bash scripts/vps-sync-vitrin-basic-auth.sh
```

Tarayıcıda site kullanıcı adı/şifre sorar; oturum tarayıcı oturumu boyunca hatırlanır.

`/assets/video/` yolu nginx’te auth dışı bırakılır; aksi halde Lertiva Arc sayfasındaki `<video>` oynatıcı boş kalabilir (alt isteklerde kimlik bilgisi gitmez).
