# Madde G9 — Motor ileri (seçili MVP)

Gap plan: [GAP_TO_100_FAZ_PLAN.md](./GAP_TO_100_FAZ_PLAN.md) · Plan: [WINLRTRAR_PLAN.md](../WINLRTRAR_PLAN.md)

## Repo teslimi (bu sprint)

- Codec matrisi: [WINLRTRAR_CODEC_MATRIX.md](./WINLRTRAR_CODEC_MATRIX.md)
- `.wlrt` tek dosya çıkarma: `wlrt::extract_one` (+ unit test)
- Perf regresyon: Madde Q2 + G1 P0 kapı
- Vitrin JSON: `lertiva-arc-motor-advanced-gate.json`

## Ürün / R&D (kapı dışı)

- Solid + deduplication, kısmi kurtarma
- Arc masaüstünde akışlı önizleme UI

## Doğrulama

```bash
bash scripts/verify-faz-g9.sh
bash scripts/evaluate-lertiva-motor-advanced-gate.sh
```
