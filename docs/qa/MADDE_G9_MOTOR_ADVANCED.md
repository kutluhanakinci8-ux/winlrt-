# Madde G9 — Motor ileri (seçili MVP)

Gap plan: [GAP_TO_100_FAZ_PLAN.md](./GAP_TO_100_FAZ_PLAN.md) · Plan: [WINLRTRAR_PLAN.md](../WINLRTRAR_PLAN.md)

## Repo teslimi (sprint 2)

- Codec matrisi: [WINLRTRAR_CODEC_MATRIX.md](./WINLRTRAR_CODEC_MATRIX.md)
- Tek dosya: `wlrt::extract_one` · `zip_io::extract_one`
- Kısmi kurtarma MVP: `zip_io::extract_partial` (+ Arc «Kısmi aç (ZIP)»)
- Dedup spike: `dedup::scan_duplicate_files` (+ `scan_create_duplicates` UI komutu)
- Arc önizleme: `preview_entry_text` (G4)
- Perf regresyon: Madde Q2 + G1 P0 kapı
- Vitrin JSON: `lertiva-arc-motor-advanced-gate.json` (`sprint2_pass`)

## Ürün / R&D (kapı dışı)

- Solid arşiv + gerçek dedup sıkıştırma
- `.wlrt` kısmi kurtarma, RAR/7z tek dosya

## Doğrulama

```bash
bash scripts/verify-faz-g9.sh
bash scripts/evaluate-lertiva-motor-advanced-gate.sh
```
