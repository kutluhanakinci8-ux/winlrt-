# Madde D — winlrtRAR Windows beta workflow %100

## Neden eksikti (~%15)?

| Eksik | Açıklama |
|-------|----------|
| **Son koşu failure** | Release adımı aynı NSIS dosyasını iki glob ile **iki kez** yüklüyordu → GitHub API 404 |
| **workflow_dispatch** | Cloud agent **403** — tetik: **push flag** |
| **Doğrulama** | Flag → artifact + (isteğe) Release yeşil kayıt |

## D %100 tanımı

1. `trigger-beta-build` push → workflow **success** + artifact `winlrt-rar-windows-bundle`
2. `trigger-beta-release` push → **success** + yeni `winlrt-rar-beta.*` Release (NSIS tek dosya)
3. `bash scripts/verify-madde-d.sh`

Tetik: `bash scripts/agent-madde-d-complete.sh` (build, sonra release).

**Doğrulandı:** run `41089469694` · release `winlrt-rar-beta.42`.
