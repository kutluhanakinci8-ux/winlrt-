# winlrtRAR — benchmark (KPI P0)

Üç KPI **paralel** izlenir (hepsi P0):

| KPI | Faz 3 hedefi |
|-----|----------------|
| Hız | Hızlı aç profili: referansa göre **≥ %30** |
| Sıkıştırma | `.wlrt` vs ZIP: **≥ %10** küçük veya eşit boyutta daha hızlı |
| Kurumsal güven | Şifreleme/imza checklist (Faz 2–4) |

## Test seti (oluşturulacak)

- `fixtures/text/` — log + UTF-8
- `fixtures/office/` — küçük örnek dosyalar
- `fixtures/binary/` — rastgele + tekrarlayan veri (dedup testi)

## Çalıştırma

```bash
cargo run -p winlrt-archive -- bench
# Genişletilmiş rapor: Faz 1 sprint 2+
```

Rakip karşılaştırması aynı makinede manuel veya CI’da dokümante edilir (WinRAR/7-Zip yolu).
