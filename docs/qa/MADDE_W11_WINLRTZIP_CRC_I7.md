# Madde W11 — winlrtZIP bozuk CRC (I7) %100

## Senaryo I7

Kısmi bozulma (ilk girdi gövdesinde tek bayt) → `test` / `extract` **kontrollü** hata:

`ZIP bütünlük hatası (CRC) — arşiv bozuk veya eksik`

## Otomasyon

| Bileşen | Konum |
|---------|--------|
| Mesaj eşleme | `zip_io::map_read_err`, `ZIP_INTEGRITY_CRC_MSG` |
| Bozma (test) | `corrupt_first_local_crc_for_test` |
| Testler | `zip_corrupt_crc_controlled_error`, `i7_*` interop |

## W11 %100

1. `bash scripts/verify-madde-w11.sh`
2. Main pipeline: `i7_` + unit `zip_corrupt_crc`
