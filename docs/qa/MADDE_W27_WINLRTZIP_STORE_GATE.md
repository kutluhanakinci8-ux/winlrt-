# Madde W27 — Lertiva Zip mağaza kapısı (Faz 7) %100

Play / App Store **ön koşul** checklist (repo); gerçek mağaza yükleme ayrı.

## Kapsam (W27)

| Özellik | Durum |
|---------|--------|
| Kapı değerlendirme | `store_gate::evaluate_store_gate` |
| Play Data safety şablon | [WINLRTZIP_STORE_DATA_SAFETY.md](./WINLRTZIP_STORE_DATA_SAFETY.md) |
| CLI | `winlrt-zip store-gate --json` |
| Vitrin örneği | `benchmark/samples/lertiva-zip-store-gate.json` |

## Bileşenler

| Bileşen | Konum |
|---------|--------|
| Motor | `crates/lertiva-archive/src/store_gate.rs` |
| Script | `scripts/evaluate-lertiva-zip-store-gate.sh` |
| Doğrulama | `scripts/verify-madde-w27.sh` |

## W27 %100

1. `bash scripts/verify-madde-w27.sh`
2. Main pipeline W27
