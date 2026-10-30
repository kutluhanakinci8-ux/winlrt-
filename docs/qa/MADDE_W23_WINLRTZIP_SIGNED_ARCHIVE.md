# Madde W23 — Lertiva Zip imzalı arşiv (Faz 4) %100

Kurumsal **Ed25519 yan dosya** imzası: `{stem}.winlrtsig.json` (W18 GPO ile birlikte).

## Kapsam (W23)

| Özellik | Durum |
|---------|--------|
| İmzala / doğrula | `sign_zip_archive`, `verify_zip_archive_signature` |
| Açma kapısı | `list` / `extract` / `test` + isteğe GPO `RequireSignedArchive` |
| CLI | `winlrt-zip sign`, `verify-sig` |
| Denetim | `archive_signature` audit olayı |
| Split ZIP | İmza ham birleşik bayt özeti üzerinden |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `archive_sign.rs`, `zip_io.rs` |
| GPO | `RequireSignedArchive` — `deploy/gpo/Lertiva Zip.*` |
| Testler | `archive_sign::*`, `w23_*` |
| Doğrulama | `scripts/verify-madde-w23.sh` |

## W23 %100

1. `bash scripts/verify-madde-w23.sh`
2. Main pipeline W23 adımı

**Not:** Kurumsal anahtar dağıtımı (PKI) ayrı süreç; seed `WINLRT_SIGNING_SEED_HEX` pilot içindir.
