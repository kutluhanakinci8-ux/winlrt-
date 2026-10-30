# Madde Q — Fast profil hız KPI (sertifika yok)

## Durum

| Profil | Kapı | CI |
|--------|------|-----|
| **Balanced** | Sıkıştırma strict | Push’ta `WINLRT_KPI_STRICT` → job fail etmez (gate true) |
| **Fast** | Hız (6 fixture) | **Strict kapalı**; `text` fixture hız henüz false (5/6) |

Vitrin: `apps/web/public/benchmark/samples/windows-fast-kpi.json` — `speed_gate: false`.

## Q %100 tanımı (bu aşama)

1. Balanced resmi strict yeşil ([Madde C](./MADDE_C_WINDOWS_BENCHMARK.md))
2. Fast adımı CI’da `-ReportOnly` (yanlış “Faz 3” uyarısı yok)
3. Eksik fixture kayıtlı → performans işi ayrı PR (bloklayıcı değil)

Doğrulama: `bash scripts/verify-madde-q.sh`
