# winlrtZIP — GPO / PolicyDefinitions (W18)

| Dosya | Hedef |
|-------|--------|
| `winlrtZIP.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\winlrtZIP.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\winlrtZIP.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` → **Yönetim Şablonları → Windows Bileşenleri → winlrtZIP**.

Örnek `.reg`: `scripts/winlrtzip-corporate-policy-sample.reg`  
Doğrulama: `scripts/windows/winlrtzip-corporate-pilot-preflight.ps1`

Vitrin ZIP: `scripts/package-winlrtzip-corporate-gpo-zip.sh`
