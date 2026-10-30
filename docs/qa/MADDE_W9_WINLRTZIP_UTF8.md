# Madde W9 — Lertiva Zip Türkçe UTF-8 dosya adları (I5) %100

## Senaryo I5

| Adım | Beklenen |
|------|----------|
| `Şirket/iğüş/Özet_2026.txt` gibi yollar | ZIP içinde aynı UTF-8 adlar |
| `list` / `extract` | İçerik birebir |
| 7-Zip (varsa) | Harici açma |
| CLI `winlrt-zip list` | Türkçe ad görünür |

Motor: `zip_entry_name_from_path` — yol **UTF-8 değilse** açık hata (I5).

## Otomasyon

| Bileşen | Konum |
|---------|--------|
| API | `zip_io::zip_entry_name_from_path` |
| Unit | `zip_io::tests::zip_turkish_utf8_entry_names` |
| Interop | `tests/winlrtzip_interop.rs` (`i5_*`) |

## W9 %100

1. `bash scripts/verify-madde-w9.sh`
2. Main pipeline: `i5_` test filtresi
