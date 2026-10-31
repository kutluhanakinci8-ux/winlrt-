# G2 ürün — EV/OV Authenticode

**Sıralı rehber:** [G2_G3_SIRALI_RUNBOOK.md](./G2_G3_SIRALI_RUNBOOK.md) (Bölüm A) · İlerleme: `bash scripts/print-g2-g3-product-progress.sh`

Dev imza tamam ([MADDE_H_IMZA_SIGNED.md](./MADDE_H_IMZA_SIGNED.md)). Kurumsal SmartScreen için:

1. EV/OV kod imzalama sertifikası (CA)
2. PFX → GitHub secret **`WINDOWS_CERTIFICATE`** (base64) + **`WINDOWS_CERTIFICATE_PASSWORD`**
3. CI: `lertiva-zip-windows.yml` / `lertiva-arc-windows.yml` → `sign-windows-nsis-bundle.ps1`
4. Onay flag: `lertiva-ev-ov-configured.flag` içeriği `ev_ov` (şablon: `.flag.example`)
5. `bash scripts/evaluate-lertiva-ev-ov-signing-gate.sh` → `all_pass: true`

Bkz. [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md) · [GITHUB_TAURI_SECRETS_KURULUM.md](./GITHUB_TAURI_SECRETS_KURULUM.md)
