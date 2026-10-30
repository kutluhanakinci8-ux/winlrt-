# Deploy SSH (GitHub Secrets only)

Canlı deploy **yalnızca** repo Secrets ile çalışır; private key repoda **tutulmaz**.

| Secret | Açıklama |
|--------|----------|
| `VPS_SSH_KEY` veya `VPS_SSH_KEY_B64` | OpenSSH private key |
| `VPS_SSH_PASSWORD` | Alternatif: root şifresi |
| `VPS_HOST`, `VPS_USER`, `VPS_PORT` | Hedef sunucu |

Mac’te tek seferlik secret kaydı:

```bash
~/.local/bin/gh secret set VPS_SSH_KEY --repo kutluhanakinci8-ux/winlrt- < ~/.ssh/winlrt-gha-deploy
```

VPS’te public key (mevcut SSH ile):

```bash
ssh -p 23422 root@37.230.105.99 'grep -q winlrt-gha-repo-deploy-dev ~/.ssh/authorized_keys 2>/dev/null || cat >> ~/.ssh/authorized_keys' < ~/.ssh/winlrt-gha-deploy.pub
```

Ayrıntı: [docs/AUTO_DEPLOY.md](../docs/AUTO_DEPLOY.md)

Public key referansı (secret değil): `winlrt-gha-deploy.pub` — repoda kalabilir.
