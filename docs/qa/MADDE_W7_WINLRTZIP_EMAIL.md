# Madde W7 — winlrtZIP e-posta eki profili (I6) %100

## Hedef (interop I6)

| Metrik | Kapı |
|--------|------|
| ZIP boyutu | ≤ **10 MiB** (`EMAIL_ATTACHMENT_MAX_ZIP_BYTES`) |
| Mixed fixture (~10 MiB ham) | `prepare.sh` sonrası email profili oluşturma |
| Süre (mixed, CI) | ≤ **120 s** (`EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS`) |

Profil: `CompressionProfile::EmailAttachment` · CLI/Tauri: `--profile email` / varsayılan winlrtZIP.

## Otomasyon

| Bileşen | Konum |
|---------|--------|
| Sabitler | `crates/winlrt-archive/src/profile.rs` |
| Testler | `crates/winlrt-archive/tests/winlrtzip_interop.rs` |
| Rapor | `scripts/log-email-profile-i6.sh` |

## W7 %100

1. `bash scripts/verify-madde-w7.sh`
2. Main pipeline: mixed prepare sonrası ignored mixed test adımı
