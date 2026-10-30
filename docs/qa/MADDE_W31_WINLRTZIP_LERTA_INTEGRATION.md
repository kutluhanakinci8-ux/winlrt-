# Madde W31 — winlrtZIP Lerta birleşik staging kapısı (Faz 8) %100

W28 posta + W29 API + W30 mesaj eki hatlarının **tek durum özeti** ve canlı gönderim ön koşulları.

## Kapsam (W31)

| Özellik | Durum |
|---------|--------|
| Kanallar | posta (`mail-attachment`) · mesaj (`message-attachment`) |
| Metadata-only | Evet — arşiv içeriği cihazda |
| Canlı gönderim | `LERTA_API_BEARER` + `WINLRT_LERTA_API_DRY_RUN` kapalı |
| CI / varsayılan | dry-run — ağ çağrısı yok |
| CLI | `lerta-integration-status` (W29 `lerta-api-status` ayrı kalır) |
| Tauri | `lerta_integration_status` |
| Schema | [lerta-integration-v1.schema.json](./lerta-integration-v1.schema.json) |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/lerta_integration.rs` |
| Testler | `lerta_integration::*`, `w31_*` |
| Doğrulama | `scripts/verify-madde-w31.sh` |

## W31 %100

1. `bash scripts/verify-madde-w31.sh`
2. Main pipeline W31
