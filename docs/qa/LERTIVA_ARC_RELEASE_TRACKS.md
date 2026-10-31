# Lertiva Arc — release pistleri (v1.0.x)

İki paralel hat: **resmi Lertiva adlı kurulum** (şu an imzasız) ve **EV/OV Authenticode** (secret hazır olunca aynı CI, ayrı operasyon).

Çoklu platform yol haritası (ARM64, macOS, Linux, mobil, mağaza): [ARC_PLATFORM_FAZ_PLAN.md](./ARC_PLATFORM_FAZ_PLAN.md).

## Track A — Resmi Windows (SmartScreen bilgilendirme)

| Alan | Değer |
|------|--------|
| Tag | `v1.0.*` (ör. `v1.0.1`) |
| CI | [.github/workflows/lertiva-arc-windows-stable.yml](../../.github/workflows/lertiva-arc-windows-stable.yml) |
| Tauri `productName` | `Lertiva Arc` |
| Kurulum adı (build) | `Lertiva Arc_{ver}_x64-setup.exe` |
| GitHub asset URL | Boşluk → nokta: `Lertiva.Arc_{ver}_x64-setup.exe` |
| İmza | Secret **yoksa** imzasız; SmartScreen “Windows tanımadı” uyarısı **beklenir** |
| Vitrin | `scripts/refresh-lertiva-downloads-manifest.sh` → gerçek asset adı |

Yayın: `bash scripts/publish-lertiva-arc-stable-tag.sh` (tag + push) veya tag push.

## Track B — EV/OV Authenticode (Madde M)

| Alan | Değer |
|------|--------|
| Secret | `WINDOWS_CERTIFICATE` / `WINDOWS_CERTIFICATE_PASSWORD` veya `TAURI_SIGNING_*` |
| CI adımı | `scripts/sign-windows-nsis-bundle.ps1` (stable workflow içinde) |
| Beta hattı | [.github/workflows/lertiva-arc-windows.yml](../../.github/workflows/lertiva-arc-windows.yml) — `winlrt-rar-beta.*` tag |
| Doğrulama | `scripts/verify-windows-signature.ps1`, `bash scripts/verify-madde-m.sh` |
| Rehber | [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md), [MADDE_M_V1_0_1_SIGNING.md](./MADDE_M_V1_0_1_SIGNING.md) |

**Politika:** v1.0.0 legacy asset’e dokunulmaz. İlk **Lertiva Arc** adlı imzalı kurulum: yeni tag (ör. `v1.0.2`) + aynı rename adımı; vitrin manifest otomatik güncellenir.

## v1.0.1 durumu (Track A)

- Release: https://github.com/kutluhanakinci8-ux/winlrt-/releases/tag/v1.0.1
- Asset: `Lertiva.Arc_1.0.1_x64-setup.exe` (label: `Lertiva Arc_1.0.1_x64-setup.exe`)
- Detay: [V1_0_1_UNSIGNED_RELEASE.md](./V1_0_1_UNSIGNED_RELEASE.md)
