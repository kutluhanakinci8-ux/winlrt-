# winlrtRAR — durum özeti (Cloud Agent)

**Son güncelleme:** Faz 3 beta · feature zinciri **`main`’de** (tip: `02ff814` civarı).

## Ne oldu?

| Alan | Durum |
|------|--------|
| Motor | `.wlrt`, ZIP AES, RAR/7z açma, benchmark KPI (6 fixture + mixed ~10 MiB) |
| Explorer | NSIS menüleri, çoklu seçim, şifreli sıkıştır, Win32 parola |
| Kurumsal | GPO/ADMX, audit mirror + HTTP forward, `RequireModernCrypto` |
| CI | Linux main pipeline + Windows benchmark **yeşil**; `winlrt-rar-windows` workflow adı/formatı ayrı kontrol |
| Vitrin | winlrt.com.tr **200** — sunucudaki `dist` GitHub deploy ile güncellenmedi (aşağı) |
| Deploy | **Blok:** `VPS_SSH_KEY`/`B64` libcrypto hatası, `VPS_SSH_PASSWORD` boş → pipeline `auth=skip` |
| PR | GitHub token ile otomatik PR/secret yazılamıyor |

## Merge

Cloud Agent **`origin/cursor/pilot-runbook-ea18` → `main`** fast-forward yaptı; [WINLRTRAR_BRANCH_MERGE.md](./WINLRTRAR_BRANCH_MERGE.md) listesindeki dallar `main` atasları.

## Deploy (otomatik devam etmek için)

1. **GitHub Secrets** (kalıcı CI): [AUTO_DEPLOY.md](./AUTO_DEPLOY.md) — `VPS_SSH_KEY_B64` veya `VPS_SSH_PASSWORD` düzeltin → `main` push veya Main pipeline.
2. **Cloud Agent secret** (hemen rsync): ortamda `VPS_SSH_KEY_B64` (+ isteğe `VPS_HOST` …) → `bash scripts/agent-deploy-vitrin.sh`
3. **İmza / beta:** `TAURI_SIGNING_*` ([CODE_SIGNING](./WINLRTRAR_CODE_SIGNING.md))

Sonraki: Windows KPI → `scripts/import-windows-kpi-to-vitrin.sh` → commit → deploy · [POST_MERGE](./WINLRTRAR_POST_MERGE.md) · [EXPLORER_QA](./WINLRTRAR_EXPLORER_QA.md)

Tam checklist: [WINLRTRAR_PRODUCTION_CHECKLIST.md](./WINLRTRAR_PRODUCTION_CHECKLIST.md)
