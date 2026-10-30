# winlrt — `main` merge sonrası sıra (15 dk checklist)

Merge tamamlandıktan sonra **bu sırayı** izleyin. Özet durum: [WINLRTRAR_DURUM.md](./WINLRTRAR_DURUM.md).

## 1. GitHub Actions (otomatik)

- [ ] **Main pipeline** — `main` push ile tetiklenir (web build + rust test + bench)
- [ ] Yeşil değilse: rust log / `mixed/prepare.sh` hatası

## 2. Secrets (bir kez)

| Secret | Kontrol |
|--------|---------|
| `VPS_*` | Deploy job gri değil; winlrt.com.tr güncellenir |
| `TAURI_SIGNING_*` | Beta build’de imza verify adımı |

Mac: `bash scripts/mac-set-github-vps-secret.sh` · `bash scripts/mac-set-github-tauri-signing-secrets.sh …`

## 3. Windows resmi KPI

- [ ] Actions → **winlrt Windows benchmark (resmi)** → Run workflow
- [ ] İsteğe: **KPI strict** veya repo Variable `WINLRT_KPI_STRICT=1`
- [ ] Artifact **bench-report-windows** indir

## 4. Vitrin KPI (Windows resmi)

```bash
# Artifact içindeki JSON yollarıyla:
bash scripts/import-windows-kpi-to-vitrin.sh ./bench-report-windows.json ./bench-report-windows-fast.json
git add apps/web/public/benchmark/samples/windows-*.json
git commit -m "chore(vitrin): Windows resmi KPI samples"
git push origin main
```

Alternatif (Linux referans, CI artifact):

- Actions → **Vitrin KPI samples (Linux)** → Run → artifact `vitrin-kpi-samples` indir → aynı klasöre kopyala → commit

## 5. winlrtRAR beta

- [ ] **winlrtRAR Windows beta build** → Release veya artifact
- [ ] Pilot PC: [WINLRTRAR_EXPLORER_QA.md](./WINLRTRAR_EXPLORER_QA.md)

## 6. Yerel smoke (isteğe bağlı)

```bash
bash scripts/post-merge-smoke.sh
```

Repo variable: `WINLRT_KPI_STRICT=1` yalnızca KPI kapısı kilitlemek için; merge sonrası ilk Windows koşusu strict **kapalı** da olabilir.
