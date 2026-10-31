# GitHub — TAURI imza secret’ları (G2)

Cloud Agent **`gh secret set` yapamaz** (403 *Resource not accessible by integration*). Kurulum **repo sahibi** veya **Actions workflow** ile yapılır.

## Yol A — Tek tık (önerilen)

1. [Actions secrets](https://github.com/kutluhanakinci8-ux/winlrt-/settings/secrets/actions) → **New repository secret**
2. **`LERTIVA_DEV_CODESIGN_PASSWORD`** — dev PFX parolası (`.github/signing/lertiva-dev-codesign.pfx.enc` ile aynı)
3. **`GITHUB_ADMIN_PAT`** — fine-grained PAT: bu repo için **Secrets: Read and write** (workflow TAURI_* yazar)
4. Actions → **Provision TAURI signing secrets (dev)** → **Run workflow** (veya `main` push)

Workflow sonrası **`TAURI_SIGNING_PRIVATE_KEY`** + **`TAURI_SIGNING_PRIVATE_KEY_PASSWORD`** oluşur; Zip/Arc Windows build otomatik tetiklenir.

## Yol B — Doğrudan (PAT gerekmez)

Windows **Authenticode** için Tauri v2 **`WINDOWS_CERTIFICATE`** kullanır (PFX base64).  
Workflow aynı değeri **`TAURI_SIGNING_PRIVATE_KEY`** adından da okur (geriye uyum).

| Secret | Değer |
|--------|--------|
| `TAURI_SIGNING_PRIVATE_KEY` veya `WINDOWS_CERTIFICATE` | PFX base64 (tek satır) |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` veya `WINDOWS_CERTIFICATE_PASSWORD` | PFX parolası |

Sonra Actions → **Lertiva Zip Windows beta build** → Run workflow.

## Doğrulama

- Log: `Verify Authenticode (secret varsa)` adımı yeşil
- `scripts/verify-windows-signature.ps1` — `signtool verify /pa`

## Cursor Cloud

Environment’a `GITHUB_ADMIN_PAT` eklersen agent `bash scripts/agent-github-push-tauri-signing.sh` çalıştırabilir.
