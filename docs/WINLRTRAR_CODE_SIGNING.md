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

- [ ] EV/OV kod imzalama sertifikası (kurum) — **v1.0.1+**
- [ ] Thumbprint veya PFX secret GitHub’da — agent 403; panel
- [x] CI imza hattı (`winlrt-rar-windows.yml`) — [Madde H](./qa/MADDE_H_IMZA.md) **%100 imzasız resmi**
- [x] İmzalı artifact doğrulama (`scripts/verify-windows-signature.ps1` — secret varsa)
- [x] Vitrin: SmartScreen / imza notu (`/downloads/`, [V1_0_0_UNSIGNED_RELEASE.md](./qa/V1_0_0_UNSIGNED_RELEASE.md))

## Hukuk

Imza, WinRAR/UnRAR lisanslarından bağımsızdır; UnRAR dağıtım onayı ayrı (bkz. RAR_READING.md).
