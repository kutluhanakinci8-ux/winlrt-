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

**Doğrulandı:** [winlrt-rar-beta.42](https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/winlrt-rar-beta.42)

| Alan | Değer |
|------|--------|
| Dosya | `winlrtRAR_0.1.0_x64-setup.exe` |
| SHA256 | `e63e6bfad5a1a317049b810c5f21ae7e4797dedecf9f09b88991bcd7354c6b33` |
| CI run (release) | `41089469694` |

Doğrulama: `bash scripts/verify-madde-i.sh`
