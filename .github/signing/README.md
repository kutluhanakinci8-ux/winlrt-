# Lertiva geliştirme kod imzası (self-signed)

Gerçek **EV/OV Authenticode** gelene kadar aynı CI hattında (`TAURI_SIGNING_*`) kullanılan **kurumsal geliştirme** sertifikası.

| Dosya | Repoda? |
|-------|---------|
| `lertiva-dev-codesign.cer` | Evet — yalnızca **public** |
| `*.pfx`, `*.key.pem` | **Hayır** — `.signing-local/` (gitignore) |

## GitHub Actions secret’ları (agent / CI)

Cloud Agent `gh secret set` yapamaz (GitHub App 403). **Otomatik kurulum:**

1. Repo secret ekle: **`LERTIVA_DEV_CODESIGN_PASSWORD`** (PFX şifresi; `lertiva-dev-codesign.pfx.enc` ile aynı).
2. Push veya Actions → **Provision TAURI signing secrets (dev)** → Run workflow.
3. Workflow `TAURI_SIGNING_PRIVATE_KEY` + `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` yazar ve Zip/Arc Windows build tetikler.

Şifreli blob: `lertiva-dev-codesign.pfx.enc` (public repoda; yalnızca parola ile açılır).

## Üretim

**Linux / macOS / agent:**

```bash
export LERTIVA_DEV_CODESIGN_PASSWORD='…'
bash scripts/generate-lertiva-dev-codesign.sh
bash scripts/push-lertiva-dev-signing-secrets.sh   # GITHUB_ADMIN_PAT veya repo owner gh
```

**Windows (PowerShell):**

```powershell
.\scripts\windows\new-lertiva-dev-codesign-cert.ps1
```

## Beklenti

- `signtool verify /pa` → **geçer** (imzalı binary).
- **SmartScreen** → bilinmeyen yayıncı uyarısı **devam eder** (EV/OV değil).
- Gerçek sertifika gelince aynı secret adlarına PFX değiştirilir; public `.cer` güncellenir.

## Güven dağıtımı (isteğe bağlı)

Kurumsal test PC’lerde public `.cer` → `LocalMachine\Root` + `TrustedPublisher` (IT). Bkz. `generate-lertiva-dev-codesign.sh` çıktısı.
