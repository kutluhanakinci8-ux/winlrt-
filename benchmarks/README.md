# Lertiva Arc — benchmark (KPI P0)

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
- `fixtures/mixed/` — ~3 MiB karışık set (`prepare.sh`; CI otomatik üretir)

## Çalıştırma

```bash
cargo run -p lertiva-archive -- bench --all
cargo run -p lertiva-archive -- bench --all --json
cargo run -p lertiva-archive -- bench --fixture benchmarks/fixtures/binary --profile maximum
```

CI: `main` push → rust job → artifact `bench-report.json` + `scripts/evaluate-bench-kpi.sh` özeti.

**Strict KPI (Windows):** Actions → *winlrt Windows benchmark (resmi)* → Run → *KPI kapısı geçmezse job başarısız* işaretle; veya repo **Variable** `WINLRT_KPI_STRICT=1` (Linux main pipeline dahil). Yerel: `WINLRT_KPI_STRICT=1 ./scripts/evaluate-bench-kpi.sh bench-report.json`.

**Vitrin örnekleri:** `bash scripts/refresh-benchmark-vitrin-samples.sh` → `apps/web/public/benchmark/samples/`.

Windows CI (`bench-windows.yml`):

```powershell
$env:WINLRT_BENCH_INCLUDE_7Z = "1"
$env:WINLRT_BENCH_INCLUDE_WINRAR = "1"
cargo run -p lertiva-archive --release -- bench --all --markdown
cargo run -p lertiva-archive --release -- bench --all --profile fast --markdown
```

JSON çıktısında `kpi` alanı Faz 3 kapı özetini içerir.
