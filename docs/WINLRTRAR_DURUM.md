# winlrtRAR — durum özeti (Cloud Agent)

**Son güncelleme:** Faz 3 beta · kod `main`’e henüz siz merge etmeden **feature dallarda**.

## Ne oldu?

| Alan | Durum |
|------|--------|
| Motor | `.wlrt`, ZIP AES, RAR/7z açma, benchmark KPI (6 fixture + mixed ~10 MiB) |
| Explorer | NSIS menüleri, çoklu seçim, şifreli sıkıştır, Win32 parola |
| Kurumsal | GPO/ADMX, audit mirror + HTTP forward, `RequireModernCrypto` |
| CI | Linux main pipeline, Windows resmi benchmark, beta build workflow |
| Vitrin | winlrt.com.tr — `/benchmark/` KPI kartları, `/downloads/` beta link |
| Deploy | VPS secret ile otomatik (repoda **deploy private key yok**) |
| PR | GitHub’da otomatik PR açılamıyor → **siz compare/merge** |

## Sizin yapmanız gerekenler (sıra)

1. **Merge:** [WINLRTRAR_BRANCH_MERGE.md](./WINLRTRAR_BRANCH_MERGE.md) veya tek link:  
   https://github.com/kutluhanakinci8-ux/winlrt-/compare/main...cursor/pilot-runbook-ea18
2. **Secrets:** `VPS_*` → vitrin deploy · `TAURI_SIGNING_*` → imzalı beta ([CODE_SIGNING](./WINLRTRAR_CODE_SIGNING.md))
3. **Actions:** Windows benchmark (strict isteğe bağlı) → beta build → Release indir
4. **Pilot:** [EXPLORER_QA](./WINLRTRAR_EXPLORER_QA.md) + kurumsal `.reg`/ADMX  
   Ayrıntılı adımlar: [WINLRTRAR_POST_MERGE.md](./WINLRTRAR_POST_MERGE.md)

## Agent’ın son dalı

- `cursor/bench-large-kpi-explorer-ea18` — mixed fixture, KPI strict variable, Explorer QA, merge rehberi
- Son dal: `cursor/post-merge-vitrin-ci-ea18` — post-merge rehber, vitrin KPI workflow  
- Sıradaki merge ucu: `cursor/pilot-runbook-ea18` — pilot kickoff + Actions runbook

Tam checklist: [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)
