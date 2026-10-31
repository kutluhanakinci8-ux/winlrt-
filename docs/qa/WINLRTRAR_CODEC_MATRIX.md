# Lertiva Arc — codec ve rakip matrisi (G9)

Motor: `crates/lertiva-archive` · Bench: `cargo run -p lertiva-archive -- bench --all`

## Desteklenen formatlar (okuma / yazma)

| Format | Okuma | Yazma | Not |
|--------|-------|-------|-----|
| `.wlrt` (zstd) | ✅ | ✅ | Birincil format |
| ZIP (zlib/deflate) | ✅ | ✅ | Lertiva Zip + motor |
| RAR (sınırlı) | ✅ | — | F2 açma; yazma yok |
| 7z | ✅ list/test | — | `sevenz` modülü |

## Bench satırları (CI / vitrin)

| Rakip | Oluşturma | Boyut | Açma | Koşul |
|-------|-----------|-------|------|--------|
| ZIP (motor) | ✅ | ✅ | ✅ | her zaman |
| 7-Zip | ✅ | ✅ | ✅ | `7z` PATH |
| WinRAR ZIP | ✅ | ✅ | ✅ | `WINLRT_BENCH_WINRAR=1` + `rar`/`WinRAR` |
| WinZip | opsiyonel | opsiyonel | opsiyonel | ortam |

P0 kapı: [MADDE_W16](./MADDE_W16_WINLRTZIP_P0_KPI.md) · G1: `lertiva-zip-p0-gate.json`

## G9 ileri maddeler (plan)

| Madde | Durum | Referans |
|-------|--------|----------|
| Akışlı tek dosya (`.wlrt` / ZIP) | ✅ `extract_one` | G9 |
| ZIP önizleme (mobil okuyucu) | ✅ `entries_preview` | W26 |
| Kısmi kurtarma (ZIP) | ✅ `extract_partial` | G9 sprint2 |
| Dedup tarama (kaynak klasör) | ✅ `dedup::scan_duplicate_files` | G9 spike |
| Solid + deduplication (sıkıştırma) | R&D | Faz 5 |
| Arc UI önizleme / tek dosya | ✅ | G4/G9 |

Doğrulama: `bash scripts/verify-madde-g9.sh`
