# Madde B — Main pipeline %100

## Neden eksikti (~%5)?

1. **`concurrency.cancel-in-progress: true`** — `main`’e arka arkaya push → önceki koşu **cancelled** (başarısız sayılmaz ama “yeşil zincir” kırılır).
2. **Deploy job** ayrı; VPS secret yoksa skip — bu **E/F** maddesi, B’nin web+rust test kapısı değil.

## B %100 tanımı

- `web` + `rust` job’ları **success** (son `main` push veya workflow_dispatch).
- Agent completion batch CI içinde yeşil.

## Yapılan düzeltme

- `main` push: **cancel-in-progress: false** (her push tam koşar).
- PR: cancel-in-progress true (eski davranış).

## Doğrulama

```bash
bash scripts/verify-main-pipeline-green.sh
```

Actions: https://github.com/kutluhanakinci8-ux/winlrt-/actions/workflows/main-pipeline.yml
