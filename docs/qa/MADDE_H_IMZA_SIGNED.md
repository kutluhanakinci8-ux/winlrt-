# Madde H (strict) — imzalı Windows CI %100 (dev Authenticode)

| Kapı | Doğrulama |
|------|-----------|
| Zip NSIS imzalı build | `lertiva-zip-windows.yml` success + log `Successfully signed` |
| Arc NSIS imzalı build | `lertiva-arc-windows.yml` success + log `Successfully signed` |
| Doğrulama adımı | `verify-windows-signature.ps1` + `WINLRT_TRUST_DEV_CODESIGN=1` |
| Secret | `TAURI_SIGNING_*` veya `WINDOWS_CERTIFICATE*` (PFX base64) |
| Vitrin özeti | `lertiva-windows-signing-gate.json` |

```bash
bash scripts/verify-madde-h-signed.sh
bash scripts/verify-faz-g2.sh
```

**EV/OV:** Aynı secret adları; SmartScreen “güvenilir yayıncı” için kurumsal sertifika gerekir.

**Vitrin ekran görüntüsü (G2+):** `apps/web/public/assets/gallery/incoming/smartscreen-lertiva.png` → build → `all_pass` vitrin alt kapısı.
