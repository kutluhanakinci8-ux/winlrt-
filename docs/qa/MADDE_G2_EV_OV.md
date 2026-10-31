# G2 ürün — EV/OV Authenticode

Dev imza tamam ([MADDE_H_IMZA_SIGNED.md](./MADDE_H_IMZA_SIGNED.md)). Kurumsal SmartScreen için:

1. EV/OV kod imzalama sertifikası (CA)
2. PFX → GitHub secret **`WINDOWS_CERTIFICATE`** (base64) + parola secret
3. CI: `lertiva-zip-windows.yml` / `lertiva-arc-windows.yml` (mevcut `sign-windows-nsis-bundle.ps1`)
4. Onay flag: `apps/web/public/benchmark/samples/lertiva-ev-ov-configured.flag` içeriği `ev_ov`
5. `bash scripts/evaluate-lertiva-ev-ov-signing-gate.sh`

Bkz. [WINLRTRAR_CODE_SIGNING.md](../WINLRTRAR_CODE_SIGNING.md)
