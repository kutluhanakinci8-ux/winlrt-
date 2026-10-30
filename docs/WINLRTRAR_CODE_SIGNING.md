# winlrtRAR imzalı kurulum (Faz 3 → v1.0)

Beta `.exe` SmartScreen uyarısı verir; **v1.0 kapısı** Authenticode imzalı MSI/NSIS.

## Tauri / Windows

`apps/winlrt-rar/src-tauri/tauri.conf.json` → `bundle.windows`:

- `certificateThumbprint` — kurumsal sertifika (User/Machine store)
- `digestAlgorithm`: `sha256`
- `timestampUrl` — DigiCert / Sectigo RFC3161

Alternatif (CI): GitHub Secrets

- `TAURI_SIGNING_PRIVATE_KEY` — `.pfx` base64 veya minisign format (Tauri dokümantasyonu)
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Workflow: `.github/workflows/winlrt-rar-windows.yml` — secret tanımlıysa `npm run tauri build` otomatik imzalar.

Mac (yapıştırma yok):

```bash
bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/sertifika.pfx
```


## Kontrol listesi

- [ ] EV/OV kod imzalama sertifikası (kurum)
- [ ] Thumbprint veya PFX secret GitHub’da
- [x] İmzalı artifact doğrulama (`scripts/verify-windows-signature.ps1` — CI’da secret varsa otomatik)
- [x] Vitrin: indirme sayfasında SmartScreen / imza notu (`/downloads/`)

## Hukuk

Imza, WinRAR/UnRAR lisanslarından bağımsızdır; UnRAR dağıtım onayı ayrı (bkz. RAR_READING.md).
