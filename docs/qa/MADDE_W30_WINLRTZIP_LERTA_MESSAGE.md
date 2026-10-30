# Madde W30 — winlrtZIP Lerta mesaj eki (Faz 8) %100

Lerta mesaj kanalı için `.wlrt` veya AES-ZIP profili; yalnızca metadata kaydı (içerik yüklenmez).

## Kapsam (W30)

| Özellik | Durum |
|---------|--------|
| Formatlar | `wlrt_v0`, `aes_zip` |
| Üst sınır | 16 MiB |
| Kayıt yolu | `/integrations/winlrt-zip/message-attachment` |
| Ortam | `LERTA_API_BASE`, `LERTA_API_BEARER`, `WINLRT_LERTA_API_DRY_RUN` |
| CI güvenliği | `CI` veya dry-run → ağ çağrısı yok |
| CLI | `lerta-message-prepare`, `lerta-message-validate`, `lerta-message-submit` |
| Tauri | `lerta_prepare_message_attachment`, `lerta_validate_message_attachment`, `lerta_submit_message_attachment` |
| Schema | [lerta-message-v1.schema.json](./lerta-message-v1.schema.json) |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/lerta_message.rs` |
| API gönderim | `lerta_api::submit_message_attachment_metadata` |
| Testler | `lerta_message::*`, `w30_*` |
| Doğrulama | `scripts/verify-madde-w30.sh` |

## W30 %100

1. `bash scripts/verify-madde-w30.sh`
2. Main pipeline W30
