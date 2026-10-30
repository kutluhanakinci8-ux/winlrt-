# winlrtRAR — benchmark (KPI P0)

Üç KPI **paralel** izlenir (hepsi P0):

| KPI | Faz 3 hedefi |
|-----|----------------|
| Hız | Hızlı aç profili: referansa göre **≥ %30** |
| Sıkıştırma | `.wlrt` vs ZIP: **≥ %10** küçük veya eşit boyutta daha hızlı |
| Kurumsal güven | Şifreleme/imza checklist (Faz 2–4) |

## Test seti

- `fixtures/text/` — log + UTF-8 (**Faz 1 başladı**)
- `fixtures/office/` — CSV örnek (UTF-8)
- `fixtures/binary/` — 64 KiB tekrarlayan bayt deseni

## Çalıştırma

```bash
cargo run -p winlrt-archive -- bench --all
cargo run -p winlrt-archive -- bench --all --json
cargo run -p winlrt-archive -- bench --fixture benchmarks/fixtures/binary --profile maximum
```

CI: `main` push → rust job → artifact `bench-report.json`.

Rakip karşılaştırması aynı makinede manuel veya CI’da dokümante edilir (WinRAR/7-Zip yolu).
