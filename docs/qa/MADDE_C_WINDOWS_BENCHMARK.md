# Madde C — Windows resmi benchmark %100

## Neden eksikti (~%30)?

| Eksik | Açıklama |
|-------|----------|
| **Strict KPI** | `WINLRT_KPI_STRICT=1` repo variable agent **403**; kapı gevşek modda |
| **Resmi koşu → vitrin** | Artifact bazen vitrine import edilmeden kalıyordu |
| **Hız kapısı (Fast)** | Windows’ta `speed_gate` hâlâ false — **P0 sıkıştırma** strict; Fast rapor (strict kapalı) |

## C %100 tanımı (bu repo)

1. **winlrt Windows benchmark (resmi)** son `main` tetiklemesi **success**
2. **Balanced** profilde **strict KPI GEÇTİ** (workflow adımı fail etmez)
3. Vitrin `windows-*-kpi.json` = son resmi artifact (`source: windows-official-ci`)
4. Doğrulama: `bash scripts/verify-madde-c.sh`

## Agent

```bash
bash scripts/agent-madde-c-complete.sh
```

Tetikleyici: `.github/flags/trigger-windows-bench` push.
