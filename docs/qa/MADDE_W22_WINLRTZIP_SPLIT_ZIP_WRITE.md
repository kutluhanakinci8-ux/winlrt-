# Madde W22 — Lertiva Zip Split ZIP yazma (Faz 5) %100

[W21](./MADDE_W21_WINLRTZIP_SPLIT_ZIP_MERGE.md) birleştirme ile uyumlu **ham cilt bölme** (üretim yolu).

## Kapsam (W22)

| Özellik | Durum |
|---------|--------|
| Cilt yazma | `split_zip_write_volumes`, `create_split_with_profile` |
| Min cilt 64 KiB | `SPLIT_ZIP_MIN_VOLUME_BYTES` |
| CLI | `winlrt-zip create-split --volume-size` |
| Info-ZIP `PK 07 08` üretimi | Harici `zip -s` · okuma [W24](./MADDE_W24_WINLRTZIP_INFOZIP_SPAN.md) |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/lertiva-archive/src/zip_io.rs` |
| CLI | `crates/lertiva-archive/src/zip_product_cli.rs` |
| Testler | `split_zip_write_*`, `w22_*` |
| Doğrulama | `scripts/verify-madde-w22.sh` |

## W22 %100

1. `bash scripts/verify-madde-w22.sh`
2. Main pipeline: `w22_` + `split_zip_write`
