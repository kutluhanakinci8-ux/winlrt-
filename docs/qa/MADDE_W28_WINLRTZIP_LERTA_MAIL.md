# Madde W28 — winlrtZIP Lerta posta eki (Faz 8) %100

W7 e-posta profili üzerine **Lerta webhook sözleşmesi** (içerik cihazda kalır).

## Kapsam (W28)

| Özellik | Durum |
|---------|--------|
| 10 MiB tier | `Standard10Mb` + EmailAttachment profili |
| 25 MiB tier | `Extended25Mb` + Balanced profili |
| SHA-256 + MIME | `prepare_lerta_mail_attachment` |
| Webhook JSON | `webhook_payload_from_prepare` |
| CLI | `lerta-mail-prepare`, `lerta-mail-validate` |
| Tauri | `lerta_prepare_mail_attachment`, `lerta_validate_mail_attachment` |
| Schema | [lerta-mail-v1.schema.json](./lerta-mail-v1.schema.json) |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/lerta_mail.rs` |
| Testler | `lerta_mail::*`, `w28_*` |
| Doğrulama | `scripts/verify-madde-w28.sh` |

## W28 %100

1. `bash scripts/verify-madde-w28.sh`
2. Main pipeline W28
