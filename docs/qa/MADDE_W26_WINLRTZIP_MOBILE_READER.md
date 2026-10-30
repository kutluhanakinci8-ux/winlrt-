# Madde W26 — winlrtZIP mobil okuyucu smoke (Faz 6) %100

Salt okuma sözleşmesi + inspect API (Android/iOS kabuk hazırlığı, Lerta intent).

## Kapsam (W26)

| Özellik | Durum |
|---------|--------|
| Yetenekler JSON | `mobile_reader::capabilities` (`create_archive: false`) |
| Inspect | `inspect` — liste, split/imza/manifest bayrakları |
| Lerta intent | `lerta_share_intent_from_zip` |
| JSON Schema | [mobile-reader-v1.schema.json](./mobile-reader-v1.schema.json) |
| CLI | `mobile-capabilities`, `mobile-inspect` |
| Tauri | `mobile_capabilities`, `mobile_inspect_zip`, `mobile_lerta_share_intent` |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/mobile_reader.rs` |
| Testler | `mobile_reader::*`, `w26_*` |
| Doğrulama | `scripts/verify-madde-w26.sh` |

## W26 %100

1. `bash scripts/verify-madde-w26.sh`
2. `cargo check -p winlrt-zip`
3. Main pipeline W26
