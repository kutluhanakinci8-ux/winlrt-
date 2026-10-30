# Madde W24 — Lertiva Zip Info-ZIP span (`PK\x07\x08`) %100

[Faz 5](./MADDE_W22_WINLRTZIP_SPLIT_ZIP_WRITE.md): `zip -s` çok parçalı arşivleri **winlrt motorunda açma** + uyumlu **yazma**.

## Kapsam (W24)

| Özellik | Durum |
|---------|--------|
| Span imza algılama | `volume_has_infozip_span_marker`, `ZIP_SPANNING_SIGNATURE` |
| Birleştirme | `normalize_merged_split_zip` (0708 kaldır + EOCD) |
| Mantıksal bayt | `merged_split_zip_bytes` (açma + imza özeti) |
| Split yazma (Info-ZIP `-s`) | Harici `zip -s`; W22 ham bölme ayrı |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/lertiva-archive/src/zip_io.rs` |
| Testler | `split_zip_infozip_*`, `w24_*` |
| Doğrulama | `scripts/verify-madde-w24.sh` |

## W24 %100

1. `bash scripts/verify-madde-w24.sh`
2. Main pipeline: `w24_` + `infozip_span`
