# Madde I — Beta build + Release doğrulama %100

## I vs D

| Madde | Odak |
|-------|------|
| **D** | Workflow tetik, artifact, CI yeşil |
| **I** | **Yayınlanmış** beta Release + NSIS asset + SHA256 |

## I %100 tanımı

1. Son `winlrt-rar-beta.*` pre-release **uploaded** NSIS (`*-setup.exe`)
2. GitHub asset **SHA256** kayıtlı (API `digest`)
3. Resmi v1.0.0 ayrı tag ([V1_0_0_UNSIGNED_RELEASE.md](./V1_0_0_UNSIGNED_RELEASE.md))

**Doğrulandı (son):** [winlrt-rar-beta.44](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.44) — `verify-madde-i.sh` her zaman en yeni beta tag’i okur.

| Alan | Örnek (beta.44) |
|------|-----------------|
| Dosya | `winlrtRAR_0.1.0_x64-setup.exe` |
| SHA256 | `de4beb6b39ae1e811b88f9284adc27308ab2968f6e62c995eddc361823312743` |
| CI run (release) | `41093461800` |

Doğrulama: `bash scripts/verify-madde-i.sh`
