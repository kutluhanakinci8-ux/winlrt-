# Lertiva Arc imzalı kurulum (Faz 3 → v1.0)

Beta `.exe` SmartScreen uyarısı verir; **v1.0 kapısı** Authenticode imzalı MSI/NSIS.

## Geliştirme imzası (self-signed — gerçek sertifika gelene kadar)

Kendi **Lertiva Dev Code Signing** sertifikamız; CI’da `TAURI_SIGNING_*` ile NSIS imzalanır. SmartScreen hâlâ “bilinmeyen yayıncı” diyebilir; **signtool verify /pa** geçer.

| Adım | Komut |
|------|--------|
| Sertifika üret (OpenSSL) | `LERTIVA_DEV_CODESIGN_PASSWORD='…' bash scripts/generate-lertiva-dev-codesign.sh` |
| Windows alternatif | `scripts/windows/new-lertiva-dev-codesign-cert.ps1` |
| GitHub Secrets | `bash scripts/push-lertiva-dev-signing-secrets.sh` veya [mac-set-github-tauri-signing-secrets.sh](./scripts/mac-set-github-tauri-signing-secrets.sh) `.pfx` ile |
| Public sertifika (repoda) | [.github/signing/lertiva-dev-codesign.cer](./.github/signing/lertiva-dev-codesign.cer) |
| Doğrulama | `bash scripts/verify-lertiva-dev-codesign.sh` |

Gerçek **EV/OV** alındığında: aynı secret adlarına kurumsal PFX yüklenir; geliştirme `.cer` isteğe bağlı kalır.

## Tauri / Windows

`apps/lertiva-arc/src-tauri/tauri.conf.json` → `bundle.windows`:

- `certificateThumbprint` — kurumsal sertifika (User/Machine store)
- `digestAlgorithm`: `sha256`
- `timestampUrl` — DigiCert / Sectigo RFC3161

Alternatif (CI): GitHub Secrets

- `TAURI_SIGNING_PRIVATE_KEY` — `.pfx` base64 veya minisign format (Tauri dokümantasyonu)
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Workflow: `.github/workflows/lertiva-arc-windows.yml` — secret tanımlıysa `npm run tauri build` otomatik imzalar.

Mac (yapıştırma yok):

```bash
bash scripts/mac-set-github-tauri-signing-secrets.sh /yol/sertifika.pfx
```


## Kontrol listesi

- [ ] EV/OV kod imzalama sertifikası (kurum) — **v1.0.1+**
- [ ] Thumbprint veya PFX secret GitHub’da — agent 403; panel
- [x] CI imza hattı (`lertiva-arc-windows.yml`) — [Madde H](./qa/MADDE_H_IMZA.md) **%100 imzasız resmi**
- [x] İmzalı artifact doğrulama (`scripts/verify-windows-signature.ps1` — secret varsa)
- [x] Vitrin: SmartScreen / imza notu (`/downloads/`, [V1_0_0_UNSIGNED_RELEASE.md](./qa/V1_0_0_UNSIGNED_RELEASE.md))

## Hukuk

Imza, WinRAR/UnRAR lisanslarından bağımsızdır; UnRAR dağıtım onayı ayrı (bkz. RAR_READING.md).
