# Madde W10 — Lertiva Zip ZIP64 (I3) %100

## Senaryo I3

| Katman | Açıklama |
|--------|----------|
| Simülasyon | `create_zip64_smoke` — `large_file(true)` + Zip64 extra |
| Doğrulama | `archive_indicates_zip64` (EOCD `PK\x06\x06` veya 0xFFFFFFFF boyut alanı) |
| Gerçek >4 GiB | `i3_sparse_over_4gib_zip_roundtrip` (`#[ignore]`, seyrek `set_len`) |

## Otomasyon

| Bileşen | Konum |
|---------|--------|
| API | `zip_io::create_zip64_smoke`, `archive_indicates_zip64`, `ZIP64_SIZE_THRESHOLD` |
| Testler | `winlrtzip_interop.rs` (`i3_*`), `zip_io::zip64_smoke_*` |

## W10 %100

1. `bash scripts/verify-madde-w10.sh`
2. Main pipeline: `i3_` + unit `zip64_smoke`
