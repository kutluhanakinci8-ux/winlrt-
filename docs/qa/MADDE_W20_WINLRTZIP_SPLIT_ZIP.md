# Madde W20 — winlrtZIP Split ZIP smoke (Faz 5) %100

[Faz 5](../WINLRTZIP_PLAN.md): çok ciltli ZIP — **algılama + smoke** (tam birleştirme sonraki sprint).

## Kapsam (W20)

| Özellik | Durum |
|---------|--------|
| EOCD span göstergesi | `archive_indicates_split_zip` |
| WinZip `.z01` ciltleri | `split_zip_volumes_on_disk` |
| Test EOCD işaretleme | `mark_split_zip_eocd_smoke_for_test` |
| Çok cilt birleştirme / açma | [W21](./MADDE_W21_WINLRTZIP_SPLIT_ZIP_MERGE.md) |
| Split yazma (üretim) | Sonraki sprint |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/winlrt-archive/src/zip_io.rs` |
| Testler | `split_zip_*`, `w20_*` interop |
| Doğrulama | `scripts/verify-madde-w20.sh` |

## W20 %100

1. `bash scripts/verify-madde-w20.sh`
2. Main pipeline: `w20_` + unit `split_zip`
