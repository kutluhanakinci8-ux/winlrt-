# Lertiva Zip — GPO / PolicyDefinitions (W18)

| Dosya | Hedef |
|-------|--------|
| `Lertiva Zip.admx` | `%SystemRoot%\PolicyDefinitions\` |
| `en-US\Lertiva Zip.adml` | `%SystemRoot%\PolicyDefinitions\en-US\` |
| `tr-TR\Lertiva Zip.adml` | `%SystemRoot%\PolicyDefinitions\tr-TR\` |

`gpupdate /force` → **Yönetim Şablonları → Windows Bileşenleri → Lertiva Zip**.

Örnek `.reg`: `scripts/winlrtzip-corporate-policy-sample.reg`  
Doğrulama: `scripts/windows/winlrtzip-corporate-pilot-preflight.ps1`

Vitrin ZIP: `scripts/package-winlrtzip-corporate-gpo-zip.sh`
