# Geliştirme deploy anahtarı (geçici)

GitHub Secret’taki `VPS_SSH_KEY` bozuksa Actions bu repodaki anahtarı kullanır.

**Canlıya geçmeden önce:** bu private key’i repodan kaldırın, yalnızca GitHub Secret kullanın.

VPS’e public key (Mac’te bir kez, mevcut ssh ile):

```bash
ssh -p 23422 -i ~/.ssh/winlrt-gha-deploy root@37.230.105.99 'grep -q winlrt-gha-repo-deploy-dev ~/.ssh/authorized_keys 2>/dev/null || echo "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIIhHbqZW6R94vUnrgz4idHdFYMIK2YteYzCWTNPB3PpG winlrt-gha-repo-deploy-dev" >> ~/.ssh/authorized_keys'
```
