# Madde W21 — winlrtZIP Split ZIP birleştirme (Faz 5) %100

[W20](./MADDE_W20_WINLRTZIP_SPLIT_ZIP.md) algılama sonrası: **çok ciltli arşivi açma** (Info-ZIP / WinZip sırası: `.z01` … `.zip`).

## Kapsam (W21)

| Özellik | Durum |
|---------|--------|
| Cilt sıralama | `split_zip_ordered_volumes` |
| Birleştirme | `SplitZipOpen` / `split_zip_materialize_merged` |
| list / extract / test | Otomatik birleştirme |
| Split yazma (üretim) | [W22](./MADDE_W22_WINLRTZIP_SPLIT_ZIP_WRITE.md) |
| Info-ZIP `zip -s` (PK 07 08) | Birleştirme hazır; tam açma sonraki iyileştirme |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/zip_io.rs` |
| Testler | `split_zip_merge_*`, `w21_*` interop |
| Doğrulama | `scripts/verify-madde-w21.sh` |

## W21 %100

1. `bash scripts/verify-madde-w21.sh`
2. Main pipeline: `w21_` + `split_zip_merge`
