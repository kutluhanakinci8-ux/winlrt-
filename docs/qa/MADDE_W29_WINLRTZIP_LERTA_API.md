# Madde W29 — winlrtZIP Lerta üretim API (Faz 8) %100

W28 webhook sözleşmesinin **lerta.com.tr** metadata kaydı (ZIP içeriği yüklenmez).

## Kapsam (W29)

| Özellik | Durum |
|---------|--------|
| Varsayılan taban URL | `https://u88.lerta.com.tr/api/v1` |
| Kayıt yolu | `/integrations/winlrt-zip/mail-attachment` |
| Ortam | `LERTA_API_BASE`, `LERTA_API_BEARER`, `WINLRT_LERTA_API_DRY_RUN` |
| CI güvenliği | `CI` veya dry-run → ağ çağrısı yok |
| CLI | `lerta-api-status`, `lerta-mail-submit` |
| Tauri | `lerta_api_status`, `lerta_submit_mail_attachment` |
| Schema | [lerta-api-v1.schema.json](./lerta-api-v1.schema.json) |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/lerta_api.rs` |
| Testler | `lerta_api::*`, `w29_*` |
| Doğrulama | `scripts/verify-madde-w29.sh` |

## W29 %100

1. `bash scripts/verify-madde-w29.sh`
2. Main pipeline W29
