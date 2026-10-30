# Çok ciltli RAR + onarım (Faz 3 beta)

## Çok cilt

Kullanıcı `backup.part03.rar` veya `data.r05` seçse bile motor **ilk cilt** yolunu türetir (`volumes.rs`):

- `*.partNN.rar` → `*.part01.rar` (veya aynı genişlikte `part1`)
- `*.rNN` → `*.rar` (aynı klasör)

Tüm ciltler **aynı dizinde** olmalıdır. UI `inspect_archive` ile ipucu gösterir.

## Onarım

- CLI: `winlrt-archive repair arşiv.rar ./onarım/ --format rar [--password]`
- Tauri: **RAR onarım dene** (UnRAR `r` komutu)
- Kurtarma kaydı olmayan arşivlerde başarısız olabilir

## Sonraki

- ZIP split / 7z multi-volume
- Gömülü libunrar ile onarım (Windows)
