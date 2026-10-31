# Madde W32 — Lertiva Zip mağaza / mobil pilot hazırlık (Post-v3) %100

Play / App Store **yükleme öncesi** birleşik repo kapısı: mağaza checklist (W27), mobil okuyucu (W26), Lerta staging (W31) ve vitrin marka dosyaları.

## Kapsam (W32)

| Bileşen | Kaynak madde |
|---------|----------------|
| Mağaza ön koşul | W27 `store_gate` |
| Mobil okuyucu | W26 `mobile_reader::capabilities` |
| Lerta staging | W31 `lerta_integration_status_from_env` |
| Vitrin marka | `apps/web/public` Lertiva Zip örnekleri |

## CLI

```bash
WINLRT_LERTA_API_DRY_RUN=1 lertiva-zip --json store-pilot-gate --workspace .
```

## Vitrin

- `apps/web/public/benchmark/samples/lertiva-zip-store-pilot-gate.json`
- Benchmark kartı: **Mağaza/mobil pilot (W32)**

## Doğrulama

1. `bash scripts/verify-madde-w32.sh`
2. Main pipeline W32

## Sonraki (W33+ aday)

- Gerçek Play / App Store yükleme + inceleme
- Android / iOS mağaza kabuğu (Faz 6 pilot)
