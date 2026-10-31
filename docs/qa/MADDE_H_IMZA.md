# Madde H — TAURI / Authenticode %100 (imzasız resmi hat)

## Neden “0%” görünüyordu?

**Geliştirme:** self-signed PFX → `TAURI_SIGNING_*` ([WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md), `scripts/generate-lertiva-dev-codesign.sh`). **Prod:** EV/OV sertifikası aynı secret adlarına geçer.

## H %100 tanımı (bu repo — v1.0.0 imzasız)

| Kapı | Durum |
|------|--------|
| CI imza hattı | `lertiva-arc-windows.yml` + `TAURI_SIGNING_*` env |
| Secret yokken | Uyarı + imzasız NSIS (beklenen) |
| Secret varken | `verify-windows-signature.ps1` koşar |
| Dokümantasyon | [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md) |
| Resmi sürüm | [V1_0_0_UNSIGNED_RELEASE.md](./V1_0_0_UNSIGNED_RELEASE.md) |
| Sonraki imzalı | `v1.0.1` + `mac-set-github-tauri-signing-secrets.sh` |

Doğrulama: `bash scripts/verify-madde-h.sh`

**Gerçek Authenticode:** panel secret → Madde H “strict” (ayrı checklist maddesi).
